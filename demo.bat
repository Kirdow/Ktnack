@echo off

set "id=%~1"

if %id%==1 (
    cargo run -- demo/01-arithmetic.ktnck
) else if %id%==2 (
    cargo run -- demo/02-if-statement.ktnck
) else if %id%==3 (
    cargo run -- demo/03-while.ktnck
) else (
    echo Table Of Content:
    echo 1 - Arithmetic
    echo 2 - If statement
    echo 3 - While
)