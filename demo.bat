@ECHO OFF

SET "id=%~1"

IF %id%==1 (
    SET "name=01-arithmetic"
    REM cargo run -- demo/01-arithmetic.ktnck
    REM ktnckc
) ELSE IF %id%==2 (
    set "name=02-if-statement"
    REM cargo run -- demo/02-if-statement.ktnck
    REM ktnckc
) ELSE IF %id%==3 (
    set "name=03-while"
    REM cargo run -- demo/03-while.ktnck
    REM ktnckc
) ELSE (
    ECHO Table Of Content:
    ECHO 1 - Arithmetic
    ECHO 2 - If statement
    ECHO 3 - While
    GOTO:EOF
)

cargo run -- demo/%name%.ktnck
%name%.exe
DEL %name%.exe
DEL %name%.asm
DEL %name%.obj
