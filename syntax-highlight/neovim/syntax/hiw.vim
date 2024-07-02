syntax keyword hiwType int str array bool

syntax match hiwComment "//*"
syntax match hiwNumber "\<\d\+\>"
syntax match hiwFunction "\<define\s\+\w\+\>"

syntax region hiwString start=+"+ end=+"+


hi def link hiwKeyword Keyword
hi def link hiwType Type
hi def link hiwComment Comment
hi def link hiwString String
hi def link hiwNumber Number
hi def link hiwFunction Function
