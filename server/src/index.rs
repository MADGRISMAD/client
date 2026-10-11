//! Índice semántico en memoria: los vectores de las vacantes, normalizados, en un solo arreglo plano.
//! Buscar es un producto punto contra todos (≈ 25 M de multiplicaciones para 100 mil vacantes de 256
//! dimensiones: milisegundos) y no necesita la extensión pgvector.
//! ponytail: vive en el proceso; con varias instancias cada una carga el suyo al arrancar y se actualiza con sus
//! propias escrituras. Si se escala a varias, mover a pgvector (HNSW) o recargar con LISTEN/NOTIFY.

use std::{cmp::Ordering, collections::HashMap, sync::RwLock};

use deadpool_postgres::Pool;
use uuid::Uuid;

pub const DIMS: usize = 256;

#[derive(Default)]
struct Inner {
    ids: Vec<Uuid>,
    data: Vec<f32>, // ids.len() * DIMS
    pos: HashMap<Uuid, usize>,
}

#[derive(Default)]
pub struct VecIndex(RwLock<Inner>);

/// Normaliza a longitud 1: así el coseno es solo el producto punto.
pub fn normalize(mut v: Vec<f32>) -> Vec<f32> {
    let n = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if n > 0.0 {
        v.iter_mut().for_each(|x| *x /= n);
    }
    v
}

pub fn dot(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

impl VecIndex {
    pub async fn load(&self, pool: &Pool) -> Result<(), Box<dyn std::error::Error>> {
        let c = pool.get().await?;
        let rows = c.query("SELECT id, embedding FROM jobs WHERE embedding IS NOT NULL", &[]).await?;
        let mut w = self.0.write().unwrap();
        for r in rows {
            let v: Vec<f32> = r.get(1);
            if v.len() == DIMS {
                Self::put(&mut w, r.get(0), &v);
            }
        }
        tracing::info!("índice semántico: {} vacantes", w.ids.len());
        Ok(())
    }

    fn put(w: &mut Inner, id: Uuid, v: &[f32]) {
        match w.pos.get(&id) {
            Some(&i) => w.data[i * DIMS..(i + 1) * DIMS].copy_from_slice(v),
            None => {
                w.pos.insert(id, w.ids.len());
                w.ids.push(id);
                w.data.extend_from_slice(v);
            }
        }
    }

    pub fn upsert(&self, id: Uuid, v: &[f32]) {
        if v.len() == DIMS {
            Self::put(&mut self.0.write().unwrap(), id, v);
        }
    }

    pub fn remove(&self, id: Uuid) {
        let mut w = self.0.write().unwrap();
        let Some(i) = w.pos.remove(&id) else { return };
        let last = w.ids.len() - 1;
        if i != last {
            let moved = w.ids[last];
            w.ids[i] = moved;
            w.pos.insert(moved, i);
            let (head, tail) = w.data.split_at_mut(last * DIMS);
            head[i * DIMS..(i + 1) * DIMS].copy_from_slice(&tail[..DIMS]);
        }
        w.ids.pop();
        w.data.truncate(last * DIMS);
    }

    /// Las `k` vacantes más parecidas (id, coseno), de mayor a menor. `q` ya viene normalizado.
    pub fn top_k(&self, q: &[f32], k: usize) -> Vec<(Uuid, f32)> {
        let r = self.0.read().unwrap();
        let mut scored: Vec<(f32, usize)> = r.data.as_chunks::<DIMS>().0.iter().enumerate().map(|(i, v)| (dot(q, v), i)).collect();
        let k = k.min(scored.len());
        if k == 0 {
            return vec![];
        }
        let cmp = |a: &(f32, usize), b: &(f32, usize)| b.0.partial_cmp(&a.0).unwrap_or(Ordering::Equal);
        if k < scored.len() {
            scored.select_nth_unstable_by(k - 1, cmp);
            scored.truncate(k);
        }
        scored.sort_by(cmp);
        scored.into_iter().map(|(s, i)| (r.ids[i], s)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unit(i: usize) -> Vec<f32> {
        let mut v = vec![0.0; DIMS];
        v[i] = 1.0;
        v
    }

    #[test]
    fn top_k_orders_by_similarity_and_survives_upsert_and_remove() {
        let ix = VecIndex::default();
        let (a, b, c) = (Uuid::now_v7(), Uuid::now_v7(), Uuid::now_v7());
        ix.upsert(a, &unit(0));
        ix.upsert(b, &unit(1));
        ix.upsert(c, &normalize({ let mut v = unit(0); v[1] = 1.0; v }));
        let q = unit(0);
        let r = ix.top_k(&q, 3);
        assert_eq!(r[0].0, a);
        assert_eq!(r[1].0, c); // la mezcla se parece más a «0» que «1»
        assert_eq!(r[2].0, b);
        assert!(r[0].1 > r[1].1 && r[1].1 > r[2].1);
        // reemplazar el vector de `b` por uno igual a la consulta lo sube al primer lugar
        ix.upsert(b, &unit(0));
        assert_eq!(ix.top_k(&q, 1).len(), 1);
        // quitar el del medio no desordena a los demás (swap_remove)
        ix.remove(a);
        let ids: Vec<Uuid> = ix.top_k(&q, 10).into_iter().map(|x| x.0).collect();
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&b) && ids.contains(&c) && !ids.contains(&a));
        ix.remove(Uuid::now_v7()); // quitar uno que no existe no hace nada
        assert_eq!(ix.top_k(&q, 10).len(), 2);
    }

    #[test]
    fn k_larger_than_index_and_empty_index() {
        let ix = VecIndex::default();
        assert!(ix.top_k(&unit(0), 5).is_empty());
        ix.upsert(Uuid::now_v7(), &unit(2));
        assert_eq!(ix.top_k(&unit(0), 50).len(), 1);
    }
}
