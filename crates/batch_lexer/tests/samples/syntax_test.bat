@echo off
setlocal EnableDelayedExpansion

:: This is a comment using ::
REM This is a comment using REM
REM =========================================
REM Test file for batch lexer implementation
REM Demonstrates various syntax features
REM =========================================

:main
    echo Starting syntax demonstration...
    
    REM Basic variable assignment
    set "test_var=Hello World"
    set "numbers=1 2 3 4 5"
    
    REM Environment variable usage
    echo Current directory is %CD%
    echo Program files is %ProgramFiles%
    
    REM Delayed expansion example
    set "counter=1"
    for %%i in (%numbers%) do (
        set /a "counter=!counter!+1"
        echo Counter is !counter!
    )
    
    REM If-else with different operators
    if "%test_var%"=="Hello World" (
        echo String comparison successful
    ) else (
        echo String comparison failed
    )
    
    if exist "%SystemRoot%\System32" (
        echo System32 directory exists
    )
    
    REM Numeric comparisons
    set /a num1=5
    set /a num2=10
    
    if %num1% LSS %num2% echo %num1% is less than %num2%
    if %num1% NEQ %num2% echo Numbers are not equal
    
    REM For loop variations
    echo Demonstrating different for loop types:
    
    REM Basic for loop
    for %%x in (A B C) do (
        echo Letter: %%x
    )
    
    REM For /L (counter)
    for /L %%n in (1,1,3) do (
        echo Number: %%n
    )
    
    REM Simulated array using for
    set "array[0]=First"
    set "array[1]=Second"
    set "array[2]=Third"
    
    for /L %%i in (0,1,2) do (
        echo Array[%%i] = !array[%%i]!
    )
    
    REM String manipulation
    set "text=Hello, World!"
    echo %text:~0,5%
    echo %text:World=Batch%
    
    REM Call demonstration
    call :SubRoutine "Param1" "Param2"
    
    REM Choice command
    choice /C YN /N /M "Would you proceed (Y/N)?"
    set "errorlevel_save=%errorlevel%"
    if %errorlevel_save%==1 (
        echo You selected Yes
    ) else (
        echo You selected No
    )
    
    REM Error handling
    call :NonExistentLabel 2>nul
    if errorlevel 1 (
        echo Previous command failed
    )
    
    goto :eof

:SubRoutine
    echo.
    echo === Subroutine Start ===
    echo First parameter: %~1
    echo Second parameter: %~2
    echo Parameter count: %*
    echo === Subroutine End ===
    exit /b 0

:ErrorHandler
    echo Error occurred in line %~1
    exit /b 1

REM === Function to demonstrate path manipulation ===
:PathManipulation
    set "test_path=C:\Directory\SubDir\file.txt"
    echo Drive: %~d1
    echo Path: %~p1
    echo Name: %~n1
    echo Extension: %~x1
    exit /b 0

endlocal
