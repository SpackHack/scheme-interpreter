# scheme-interpreter

Scheme Interpreter in Rust

## Options

- -f 'Filepath' run file
- -i run init.scm
- -t show eval time

## ScmObject

| type   | input             |
| ------ | ----------------- |
| true   | **#T** or **#t**  |
| false  | **#F** or **#f**  |
| Null   | **#N** or **#n**  |
| Number | ...,-1, 0, 1, ... |
| String | "string value"    |

## Syntax

- quote
- define
- set
- lambda
- if
- begin

## Buildinfunctions

| name         | return    |            |
| ------------ | --------- | ---------- |
| \+           | number    | add number |
| \-           | number    |            |
| \*           | number    |            |
| display      | Void      |            |
| print        | Void      |            |
| print-env    | Void      |            |
| cons         | cons      |            |
| car          | ScmObject |            |
| cdr          | ScmObject |            |
| eq           | bool      |            |
| \>           | bool      |            |
| string?      | bool      |            |
| cons?        | bool      |            |
| number?      | bool      |            |
| is-buildinfn |           |            |
| is-syntax    | bool      |            |
| is-fn        | bool      |            |
| symbol?      | bool      |            |
| =            | bool      |            |
| fn-body      | list      |            |
| fn-arg       | list      |            |
| list         | list      |            |
| load         |           |            |
| open         |           |            |
| close        |           |            |
| read         |           |            |
| read-char    |           |            |
| read-line    |           |            |

## Function in Init.scm

| name     | return |     |
| -------- | ------ | --- |
| not      | bool   |     |
| <        | bool   |     |
| >=       | bool   |     |
| <=       | bool   |     |
| for-loop | void   |     |
