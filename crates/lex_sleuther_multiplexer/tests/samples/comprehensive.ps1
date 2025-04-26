# This is a comprehensive PowerShell syntax test file
# It demonstrates various PowerShell language features

# Variables and assignment
$Global:GlobalVar = "Global variable"
$script:ScriptVar = "Script-scoped variable"
$private:PrivateVar = 42
$arr = @(1, 2, 3)
$hash = @{
    Key1 = "Value1"
    Key2 = 2
    "Key 3" = "Value 3"
}

# Function definition with parameters and type declarations
function Test-Function {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory=$true)]
        [string]$Name,
        
        [Parameter()]
        [int]$Age = 30,
        
        [ValidateSet("A", "B", "C")]
        [string]$Option
    )
    
    Write-Output "Hello, $Name! Age: $Age, Option: $Option"
}

# Here-string
$multiLine = @"
This is a multi-line
string using here-string
syntax with $variables
"@

# Pipeline and cmdlets
Get-Process | 
    Where-Object { $_.CPU -gt 10 } |
    Select-Object Name, CPU |
    Sort-Object CPU -Descending

# Error handling
try {
    throw "Test error"
} catch {
    Write-Error $_.Exception.Message
} finally {
    Write-Output "Cleanup code here"
}

# Classes and inheritance
class Animal {
    [string]$Name
    [int]$Age
    
    Animal([string]$name) {
        $this.Name = $name
    }
    
    [string]MakeSound() {
        return "Generic animal sound"
    }
}

class Dog : Animal {
    Dog([string]$name) : base($name) {}
    
    [string]MakeSound() {
        return "Woof!"
    }
}

# Switch statement with regex
$text = "test123"
switch -Regex ($text) {
    "^test" { "Starts with test" }
    "\d+$" { "Ends with numbers" }
    default { "No match" }
}

# Advanced operators
$x = 5
$y = 10
$result = $x -lt $y -and $x -gt 0
$contains = "Hello World" -like "*World*"
$matches = "Test123" -match "^\w+\d+$"

# Splatting
$params = @{
    Path = "test.txt"
    Encoding = "UTF8"
    Force = $true
}
# Get-Content @params

# Filter script block
filter Get-EvenNumbers {
    if ($_ % 2 -eq 0) {
        $_
    }
}

# Advanced function with parameter validation
function Test-Advanced {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory=$true)]
        [ValidateNotNullOrEmpty()]
        [string]$InputString,
        
        [Parameter()]
        [ValidateRange(1,100)]
        [int]$Count = 1,
        
        [switch]$Force
    )
    
    process {
        1..$Count | ForEach-Object {
            Write-Output $InputString
        }
    }
}

# Array manipulation
$array = @("one", "two", "three")
$array += "four"
$array[1..2]
$array -join ", "

# Type casting and conversion
[int]"42"
[string]123
[bool]1
[datetime]"2023-01-01"

# Format operator
"{0:N2}" -f 123.456789
"Value is {0:C}" -f 42.50

# Exit the script (commented out for testing)
# exit 0
