# scheme-interpreter

Scheme Interpreter in Rust

## Options

- -f 'Filepath' run file
- -i run init.scm
- -t show eval time
- -h show help

## ScmObject

| type         | type | input              |     |
| ------------ | ---- | ------------------ | --- |
| Error        |      | -                  |     |
| Integer      |      | 1                  |     |
| Float        |      | 1.0                |     |
| Chars        |      | ""                 |     |
| Cons         |      | with function list |     |
| Nil          |      | -                  |     |
| Symbole      |      | **#T** or **#t**   |     |
| Function     |      | -                  |     |
| Syntax       |      | -                  |     |
| UserFunction |      | with syntax lambda |     |
| EndOfFile    |      | -                  |     |
| Null         |      | **#N** or **#n**   |     |
| Void         |      | **#V** or **#v**   |     |
| true         |      | **#T** or **#t**   |     |
| false        |      | **#F** or **#f**   |     |
| Env          |      | -                  |     |
| Stream       |      | with function open |     |

## Syntax

| name   | return         | Parameter          |     |
| ------ | -------------- | ------------------ | --- |
| quote  | ScmObject      | ScmObject          |     |
| define | Void           | Symbole, ScmObject |     |
| set    | Void           | Symbole, ScmObject |     |
| lambda | UserFunction   | Cons, Cons         |     |
| if     | -              | bool, Cons, Cons   |     |
| begin  | last Parameter | ScmObject 0..n     |     |

## Functions

| name          | return      | Parameter              |                   |
| ------------- | ----------- | ---------------------- | ----------------- |
| \+            | number      | Integer or Float 0..n  |                   |
| \-            | number      | Integer or Float 0..n  |                   |
| \*            | number      | Integer or Float 0..n  |                   |
| /             | number      | Integer or Float 0..n  |                   |
| \>            | bool        | Two Integers or Floats |                   |
| =             | bool        | Two Integers or Floats |                   |
| display       | Void        | ScmObject 0..n         |                   |
| print         | Void        | ScmObject 0..n         |                   |
| print-env     | Void        | -                      |                   |
| cons          | cons        | Two ScmObject          |                   |
| car           | ScmObject   | Cons                   |                   |
| cdr           | ScmObject   | Cons                   |                   |
| eq?           | bool        | Two ScmObject          |                   |
| string?       | bool        | ScmObject              |                   |
| cons?         | bool        | ScmObject              |                   |
| number?       | bool        | ScmObject              |                   |
| integer?      | bool        | ScmObject              |                   |
| float?        | bool        | ScmObject              |                   |
| function?     | bool        | ScmObject              |                   |
| symbol?       | bool        | ScmObject              |                   |
| null?         | bool        | ScmObject              |                   |
| string-length | integer     | String                 |                   |
| string=?      | bool        | Two String             |                   |
| string-append | integer     | String 0..n            |                   |
| length        | integer     | Cons                   |                   |
| append        | integer     | Two Cons               |                   |
| fn-body       | Cons        | UserFunction           |                   |
| fn-arg        | Cons        | UserFunction           |                   |
| list          | Cons        | ScmObject 0..n         |                   |
| load          | Void        | String                 |                   |
| open          | Stream      | String                 |                   |
| read-line     | a Data type | Stream                 |                   |
| close         | Void        | Stream                 |                   |
| random        | Integer     | Integer                |                   |
| exit          | -           | -                      | exit the programm |

## Function in Init.scm

| name     | return |                            |                  |
| -------- | ------ | -------------------------- | ---------------- |
| not      | bool   | bool                       |                  |
| <        | bool   | Two Integers or Floats     |                  |
| >=       | bool   | Two Integers or Floats     |                  |
| <=       | bool   | Two Integers or Floats     |                  |
| !=       | bool   | Two Integers or Floats     |                  |
| equal?   | bool   | Two ScmObject              |                  |
| for-loop | void   | Integer, Integer, Function |                  |
| sum-to   | -      | Integer                    |                  |
| reload   | -      |                            | run init.scm     |
| selftest | -      |                            | run selftest.scm |
| game     | -      |                            | run game.scm     |

## game.scm

Guess the Number Game in Scheme
