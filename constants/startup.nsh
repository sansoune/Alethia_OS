@echo -off
mode 80 25

cls
if exist .\efi\boot\alethia_os.efi then
 .\efi\boot\alethia_os.efi
 goto END
endif

if exist fs0:\efi\boot\alethia_os.efi then
 fs0:
 echo Found bootloader on fs0:
 fs0:\efi\boot\alethia_os.efi
 goto END
endif

if exist fs1:\efi\boot\alethia_os.efi then
 fs1:
 echo Found bootloader on fs1:
 efi\boot\alethia_os.efi
 goto END
endif

if exist fs2:\efi\boot\alethia_os.efi then
 fs2:
 echo Found bootloader on fs2:
 efi\boot\alethia_os.efi
 goto END
endif

if exist fs3:\efi\boot\alethia_os.efi then
 fs3:
 echo Found bootloader on fs3:
 efi\boot\alethia_os.efi
 goto END
endif

if exist fs4:\efi\boot\alethia_os.efi then
 fs4:
 echo Found bootloader on fs4:
 efi\boot\alethia_os.efi
 goto END
endif

if exist fs5:\efi\boot\alethia_os.efi then
 fs5:
 echo Found bootloader on fs5:
 efi\boot\alethia_os.efi
 goto END
endif

if exist fs6:\efi\boot\alethia_os.efi then
 fs6:
 echo Found bootloader on fs6:
 efi\boot\alethia_os.efi
 goto END
endif

if exist fs7:\efi\boot\alethia_os.efi then
 fs7:
 echo Found bootloader on fs7:
 efi\boot\alethia_os.efi
 goto END
endif

 echo "Unable to find bootloader".
 
:END