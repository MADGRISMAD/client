# Prueba de la IA contra un Gemini de mentira (scripts/fake_gemini.py). Ver README.
import json, time, urllib.request, urllib.error
exec(open(__file__.replace('smoke_ai.py','smoke.py')).read().split('# ---- pruebas ----')[0])
call('POST','/users/register',{'role':'student','fullName':'Ana','email':'ana@uni.edu','password':'secreto123','degree':'Diseño','university':'UNAM','skills':['Figma','UX']})
call('POST','/users/register',{'role':'employer','fullName':'Luis','email':'luis@acme.com','password':'secreto123','company':{'name':'ACME'}})
st=call('POST','/users/login',{'email':'ana@uni.edu','password':'secreto123'})[1]['token']
et=call('POST','/users/login',{'email':'luis@acme.com','password':'secreto123'})[1]['token']
call('POST','/jobs',{'title':'Diseñador UX','description':'Interfaces en Figma para apps','tags':['Figma','UX'],'isRemote':True},et)
call('POST','/jobs',{'title':'Analista de datos','description':'Python y SQL para reportes','tags':['Python','SQL']},et)
call('POST','/jobs',{'title':'Cocinero','description':'Cocina de restaurante','tags':['Cocina']},et)
time.sleep(1.0)  # los vectores se calculan en segundo plano
s,r=call('POST','/ai/search',{'query':'quiero algo de diseño'},st); ok(s==200 and r['results'][0]['title']=='Diseñador UX' and r['results'][0]['matchScore']>r['results'][-1]['matchScore'],'búsqueda semántica: «diseño» -> %s (%s)'%(r['results'][0]['title'],[ (j['title'],j['matchScore']) for j in r['results']]))
s,r=call('POST','/ai/search',{'query':'datos y python','remote':False},st); ok(s==200 and r['results'][0]['title']=='Analista de datos' and all(not j['isRemote'] for j in r['results']),'búsqueda + filtro remoto')
s,r=call('POST','/ai/search',{'query':'x'},st); ok(s==400,'consulta muy corta -> 400')
s,r=call('POST','/ai/search',{'query':'diseño'}); ok(s==401,'búsqueda IA exige sesión')
time.sleep(0.5)
s,r=call('GET','/ai/recommended',None,st); ok(s==200 and r['results'][0]['title']=='Diseñador UX','recomendadas para el perfil: %s'%[j['title'] for j in r['results']])
jid=call('GET','/jobs?q=dise%C3%B1ador')[1][0]['_id']
s,r=call('GET',f'/jobs/{jid}/match',None,st); ok(s==200 and r['aiUsed'] and r['score']>50,'match con IA: %s'%r)
s,r=call('POST','/ai/improve-job',{'title':'Diseñador','description':'hace cosas'},et); ok(s==200 and r['tags']==['Figma','UX'],'asistente del empleador')
s,r=call('POST','/ai/improve-job',{'title':'Diseñador','description':'x'},st); ok(s==403,'asistente solo empleadores')
s,r=call('POST','/ai/cover-letter',{'jobId':jid},st); ok(s==200 and 'borrador' in r['coverLetter'],'carta de presentación')
# editar una vacante recalcula su vector
call('PUT',f'/jobs/{jid}',{'title':'Cocinero de sushi','description':'cocina japonesa','tags':['Cocina']},et); time.sleep(1.0)
s,r=call('POST','/ai/search',{'query':'cocina japonesa'},st); ok(r['results'][0]['title']=='Cocinero de sushi','tras editar, el índice se actualizó: %s'%r['results'][0]['title'])
s,r=call('POST','/ai/search',{'query':'diseño figma ux'},st); ok([j['matchScore'] for j in r['results'] if j['title']=='Cocinero de sushi']==[0],'y ya no se parece a «diseño»')
call('DELETE',f'/jobs/{jid}',None,et)
s,r=call('POST','/ai/search',{'query':'cocina japonesa'},st); ok(all(j['_id']!=jid for j in r['results']),'una vacante borrada sale del índice')
print('\nFALLAS:',ok.bad)
