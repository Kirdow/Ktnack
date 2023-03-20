@ECHO OFF

SET "id=%~1"

IF %id%==1 (
    SET "name=01-arithmetic"
) ELSE IF %id%==2 (
    SET "name=02-if-statement"
) ELSE IF %id%==3 (
    SET "name=03-while"
) ELSE IF %id%==4 (
    SET "name=04-mem"
) ELSE (
    ECHO Table Of Content:
    ECHO 1 - Arithmetic
    ECHO 2 - If statement
    ECHO 3 - While
    ECHO 4 - Memory
    GOTO:EOF
)

cargo run -- demo/%name%.ktnck
%name%.exe
DEL %name%.exe
DEL %name%.asm
DEL %name%.obj
