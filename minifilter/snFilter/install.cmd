echo Removing old version if exist > "%~dp0\install.log"
start /wait /b sc stop snFilter >> "%~dp0\install.log" 2>&1
start /wait /b sc delete snFilter >> "%~dp0\install.log" 2>&1
start /wait /b pnputil -d %~dp0\snFilter.inf >> "%~dp0\install.log" 2>&1

echo Installing new version >> "%~dp0\install.log" 2>&1
start /wait /b pnputil -i -a %~dp0\snFilter.inf >> "%~dp0\install.log" 2>&1
start /wait /b sc start snFilter >> "%~dp0\install.log" 2>&1
