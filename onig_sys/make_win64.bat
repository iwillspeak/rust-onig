SET ONIG_DIR=%~dp0\oniguruma\src
set THIS_DIR=%~dp0
set BUILD_DIR=%cd%
copy %ONIG_DIR%\config.h.win64 %BUILD_DIR%\config.h
nmake -f %THIS_DIR%\Makefile.windows
