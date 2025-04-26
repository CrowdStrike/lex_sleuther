Option Explicit
' This is a single-line comment

#Const DEBUG = True
#If DEBUG Then
    Const LOGGING_ENABLED = True
#Else
    Const LOGGING_ENABLED = False
#End If

' Enum declaration
Public Enum Colors
    Red = 1
    Green = 2
    Blue = 3
    Custom = &H100
End Enum

' Type declaration
Private Type Customer
    ID As Long
    Name As String * 50
    Balance As Currency
    IsActive As Boolean
End Type

' Module-level variables
Private m_customers() As Customer
Private m_lastError As String

' Function with return value
Public Function CalculateTotal(ByVal price As Double, Optional ByVal quantity As Integer = 1) As Double
    On Error GoTo ErrorHandler
    
    If quantity <= 0 Then
        Err.Raise vbObjectError + 1000, "CalculateTotal", "Quantity must be positive"
    End If
    
    CalculateTotal = price * quantity
    Exit Function
    
ErrorHandler:
    m_lastError = Err.Description
    CalculateTotal = -1
End Function

' Sub procedure with different parameter passing modes
Public Sub ProcessCustomer(ByVal custName As String, ByRef success As Boolean)
    Dim localVar As Variant
    Dim i As Integer
    
    success = False
    
    ' String operations
    If Len(Trim$(custName)) = 0 Then Exit Sub
    
    ' Select Case demonstration
    Select Case LCase$(Left$(custName, 1))
        Case "a" To "m"
            localVar = "First Half"
        Case "n" To "z"
            localVar = "Second Half"
        Case Else
            localVar = "Special"
    End Select
    
    ' Loop demonstrations
    For i = 1 To 10 Step 2
        Debug.Print i
    Next i
    
    Dim counter As Long
    counter = 0
    
    Do While counter < 5
        counter = counter + 1
        If counter = 3 Then
            Exit Do
        End If
    Loop
    
    ' Array manipulation
    Dim numbers(5) As Integer
    For i = 0 To UBound(numbers)
        numbers(i) = i * 2
    Next
    
    ' Collection usage
    Dim col As New Collection
    col.Add "Item 1"
    col.Add "Item 2", "key2"
    
    success = True
End Sub

' Property with validation
Private m_value As Long
Public Property Get Value() As Long
    Value = m_value
End Property

Public Property Let Value(ByVal newValue As Long)
    If newValue >= 0 Then
        m_value = newValue
    Else
        Err.Raise 513, "Value", "Value must be non-negative"
    End If
End Property

' Event declaration
Public Event StatusChanged(ByVal newStatus As String)

' API declaration
Private Declare Function GetTickCount Lib "kernel32" () As Long

' Initialize module
Private Sub Class_Initialize()
    ReDim m_customers(10)
    m_lastError = ""
End Sub
