# Prueba de punta a punta de la API (requiere el servidor en marcha y una base vacía). Ver README.
import json, urllib.request, urllib.error
import os, urllib.parse
B=os.environ.get('API','http://localhost:4000')+'/api'
def call(method, path, body=None, tok=None):
    req=urllib.request.Request(B+path, method=method, data=json.dumps(body).encode() if body is not None else None, headers={'Content-Type':'application/json', **({'Authorization':'Bearer '+tok} if tok else {})})
    try:
        r=urllib.request.urlopen(req); t=r.read(); return r.status, (json.loads(t) if t else None)
    except urllib.error.HTTPError as e:
        t=e.read(); return e.code, (json.loads(t) if t else None)
def ok(cond, msg):
    print(('OK   ' if cond else 'FALLA'), msg)
    if not cond: ok.bad+=1
ok.bad=0
# ---- pruebas ----
s,_=call('POST','/users/register',{'role':'student','fullName':'Ana Pérez','email':'ana@uni.edu','password':'secreto123','university':'UNAM','degree':'Diseño','skills':['Figma','Photoshop']}); ok(s==201,'registro estudiante .edu')
s,r=call('POST','/users/register',{'role':'student','fullName':'X','email':'x@gmail.com','password':'secreto123'}); ok(s==400 and '.edu' in r['message'],'estudiante sin .edu rechazado')
s,r=call('POST','/users/register',{'role':'student','fullName':'Ana','email':'ANA@uni.edu','password':'secreto123'}); ok(s==409,'correo repetido (sin importar mayúsculas) -> 409')
s,r=call('POST','/users/register',{'role':'employer','fullName':'Luis Jefe','email':'luis@acme.com','password':'secreto123','company':{'name':'ACME','website':'https://acme.mx','description':'x'}}); ok(s==201,'registro empleador')
s,r=call('POST','/users/login',{'email':'ana@uni.edu','password':'mala'}); ok(s==400,'login con contraseña mala -> 400')
s,r=call('POST','/users/login',{'email':'ana@uni.edu','password':'secreto123'}); ok(s==200 and r['user']['role']=='student' and 'password' not in r['user'],'login estudiante, sin exponer contraseña'); st=r['token']; sid=r['user']['_id']
s,r=call('POST','/users/login',{'email':'luis@acme.com','password':'secreto123'}); et=r['token']
s,r=call('POST','/jobs',{'title':'Diseñador UX','description':'Diseñarás interfaces en Figma para apps móviles'},st); ok(s==403,'estudiante no puede publicar')
s,r=call('POST','/jobs',{'title':'Diseñador UX','description':'Diseñarás interfaces en Figma para apps móviles','tags':['Figma','UX'],'isRemote':True,'salaryRange':{'min':10,'max':20,'type':'hora','currency':'USD'}},et); ok(s==201 and r['company']=='ACME' and r['salaryRange']['max']==20,'empleador publica (empresa tomada del perfil)'); jid=r['_id']
call('POST','/jobs',{'title':'Analista de datos','description':'Limpieza y análisis con Python y SQL','tags':['Python','SQL']},et)
call('POST','/jobs',{'title':'Cocinero','description':'Cocina de restaurante','isRemote':False},et)
s,r=call('GET','/jobs'); ok(s==200 and len(r)==3,'lista pública: 3 vacantes')
for q,exp in [('diseñador','Diseñador UX'),('diseñador','Diseñador UX'),('disenador','Diseñador UX'),('Disenador ux','Diseñador UX'),('analisis datos','Analista de datos'),('Analsta','Analista de datos'),('python','Analista de datos')]:
    s,r=call('GET','/jobs?q='+urllib.parse.quote(q)); ok(s==200 and r and r[0]['title']==exp, f'búsqueda «{q}» -> {r[0]["title"] if r else None}')
s,r=call('GET','/jobs?remote=true'); ok(len(r)==1 and r[0]['title']=='Diseñador UX','filtro remoto')
s,r=call('GET','/jobs?minSalary=15'); ok(len(r)==1,'filtro salario mínimo')
s,r=call('GET','/jobs/'+jid); ok(s==200 and r['_id']==jid,'detalle por id')
s,r=call('GET','/jobs/00000000-0000-0000-0000-000000000000'); ok(s==404,'id inexistente -> 404')
s,r=call('GET','/jobs/no-es-uuid'); ok(s in (400,404),'id inválido no revienta (%s)'%s)
s,r=call('POST',f'/jobs/{jid}/apply',{'coverLetter':'Hola, me interesa'},st); ok(s==201,'estudiante se postula')
s,r=call('POST',f'/jobs/{jid}/apply',{'coverLetter':'otra vez'},st); ok(s==409 and 'Ya te postulaste' in r['message'],'postulación repetida -> 409')
s,r=call('POST',f'/jobs/{jid}/apply',{},et); ok(s==403,'empleador no puede postularse')
s,r=call('GET','/jobs/my-jobs',None,et); ok(s==200 and len(r)==3 and any(j['applicantsCount']==1 for j in r),'mis vacantes con conteo de postulantes')
s,r=call('GET',f'/jobs/{jid}/applicants',None,et); ok(s==200 and r[0]['user']['fullName']=='Ana Pérez' and r[0]['status']=='applied','empleador ve postulantes'); aid=r[0]['_id']
s,r=call('GET',f'/jobs/{jid}/applicants',None,st); ok(s==403,'estudiante no ve postulantes')
s,r=call('PUT',f'/jobs/{jid}/applicants/{aid}',{'status':'interview'},et); ok(s==200,'empleador cambia estado a entrevista')
s,r=call('PUT',f'/jobs/{jid}/applicants/{aid}',{'status':'xx'},et); ok(s==400,'estado inválido -> 400')
s,r=call('GET','/jobs/my-applications',None,st); ok(s==200 and r[0]['status']=='interview' and 'appliedAt' in r[0],'mis postulaciones con estado')
s,r=call('GET','/notifications',None,st); ok(s==200 and any('entrevista' in n['message'] for n in r),'estudiante recibe aviso del cambio'); nid=r[0]['_id']
s,r=call('GET','/notifications',None,et); ok(any('Ana' in n['message'] for n in r),'empleador recibe aviso de postulación')
s,r=call('PUT',f'/notifications/{nid}/read',None,st); ok(s==204,'marcar leída')
s,r=call('PUT','/notifications/mark-all',None,et); ok(s==204,'marcar todas')
s,r=call('GET','/notifications'); ok(s==401,'sin token -> 401')
s,r=call('GET','/notifications',None,'basura'); ok(s==401,'token falso -> 401')
s,r=call('PUT','/users/me',{'fullName':'Ana P.','skills':['Figma','UX','  ']},st); ok(s==200 and r['skills']==['Figma','UX'],'editar perfil (limpia habilidades vacías)')
s,r=call('GET',f'/jobs/{jid}/match',None,st); ok(s==200 and r['aiUsed']==False and r['matchedSkills'] and 'Figma' in r['matchedSkills'],'match sin IA: por habilidades %s'%r)
s,r=call('POST','/ai/search',{'query':'diseño'},st); ok(s==503,'IA apagada -> 503 claro: %s'%(r or {}).get('message'))
s,r=call('PUT',f'/jobs/{jid}',{'title':'Diseñador UX Sr','description':'nueva'},st); ok(s==403,'estudiante no edita')
s,r=call('PUT',f'/jobs/{jid}',{'title':'Diseñador UX Sr','description':'nueva descr'},et); ok(s==200 and r['title']=='Diseñador UX Sr','empleador edita su vacante')
s,r=call('DELETE',f'/jobs/{jid}',None,et); ok(s==204,'empleador borra'); s,r=call('GET','/jobs'); ok(len(r)==2,'ya no aparece')
print('\nFALLAS:',ok.bad)
