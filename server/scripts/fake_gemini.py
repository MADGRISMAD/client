# Gemini de mentira: vectores por bolsa de palabras (sin acentos), para probar el flujo sin red ni llave.
import json, re, unicodedata, hashlib, math
from http.server import BaseHTTPRequestHandler, HTTPServer
DIMS=256
def words(t):
    t=unicodedata.normalize('NFD',t.lower()); t=''.join(c for c in t if unicodedata.category(c)!='Mn')
    return re.findall(r'[a-z0-9]+',t)
# sinónimos para que «diseño» se parezca a «ux»/«figma»: lo que la IA real hace por significado
SYN={'diseno':['ux','figma','interfaces'],'ux':['diseno'],'figma':['diseno'],'datos':['python','sql','analisis'],'python':['datos'],'sql':['datos']}
def emb(t):
    v=[0.0]*DIMS
    for w in words(t):
        for x in [w]+SYN.get(w,[]):
            v[int(hashlib.md5(x.encode()).hexdigest(),16)%DIMS]+=1
    n=math.sqrt(sum(a*a for a in v)) or 1
    return [a/n for a in v]
class H(BaseHTTPRequestHandler):
    def log_message(self,*a): pass
    def do_POST(self):
        b=json.loads(self.rfile.read(int(self.headers['Content-Length'])))
        assert self.headers.get('x-goog-api-key')=='fake-key'
        if self.path.endswith(':embedContent'):
            out={'embedding':{'values':emb(b['content']['parts'][0]['text'])}}
            assert b['outputDimensionality']==DIMS
        else:
            prompt=b['contents'][0]['parts'][0]['text']
            if 'responseMimeType' in b['generationConfig']:
                out={'candidates':[{'content':{'parts':[{'text':json.dumps({'description':'Descripción pulida','tags':['Figma','UX'],'requirements':['Portafolio'],'responsibilities':['Diseñar'],'benefits':['Flexible']})}]}}]}
            else:
                out={'candidates':[{'content':{'parts':[{'text':'Hola, soy Ana y me interesa la vacante. (borrador)'}]}}]}
        d=json.dumps(out).encode(); self.send_response(200); self.send_header('Content-Type','application/json'); self.send_header('Content-Length',str(len(d))); self.end_headers(); self.wfile.write(d)
HTTPServer(('127.0.0.1',4999),H).serve_forever()
