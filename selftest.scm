; tab setting: 8

;/////////////////////////////////////////////////////////////////
;/
;/ Copyright 2020 Claus Gittinger
;/
;/ You may use this, but not claim to have written or own it!
;/ Use at your own risk.
;/
;/////////////////////////////////////////////////////////////////

(display "**************************** start of selftest.scm **************\n")

(define (nameOrUnnamedIfNil arg)
    (if (eq? arg '()) "unnamed" arg))

(define (ASSERT_L lineNr boolean nameOfTest)
    (if boolean
	(begin
	    #void ; (display (nameOrUnnamedIfNil nameOfTest) ": ok\n")
	)
    ; else
	(begin
	    (display (nameOrUnnamedIfNil nameOfTest) ": **** FAILED [" lineNr "]\n")
	    (abort)
	)
    )
)

(define (ASSERT boolean nameOfTest)
    (ASSERT_L "?" boolean nameOfTest))

(define (ASSERT_RSLT function expected)
    (define result (function))


    (if (equal? result expected)
	(begin
	    ; (display (nameOrUnnamedIfNil (function-name function)))
	    #void ; (display ": ok\n")
	)
    ;else
	(begin
	    (display (nameOrUnnamedIfNil (function-name function)))
	    (display ": **** FAILED (expected:" expected " got:" result "\n")
	)
    )
)

;; ----------------
;; predicates
;; ----------------

;;
;; symbol?
;;
(ASSERT (eq? (symbol? "") #f) "symbol?")
(ASSERT (eq? (symbol? 'a) #t) "symbol?")
(ASSERT (eq? (symbol? "1") #f) "symbol?")
(ASSERT (eq? (symbol? '()) #f) "symbol?")
(ASSERT (eq? (symbol? 1234) #f) "symbol?")
(ASSERT (eq? (symbol? 1234.5) #f) "symbol?")
(ASSERT (eq? (symbol? '(1 2 3)) #f) "symbol?")

;;
;; cons?
;;
(ASSERT (eq? (cons? "") #f) "cons?")
(ASSERT (eq? (cons? 'a) #f) "cons?")
(ASSERT (eq? (cons? "1") #f) "cons?")
(ASSERT (eq? (cons? '()) #f) "cons?")
(ASSERT (eq? (cons? 1234) #f) "cons?")
(ASSERT (eq? (cons? 1234.5) #f) "cons?")
(ASSERT (eq? (cons? '(1 2 3)) #t) "cons?")

;;
;; string?
;;
(ASSERT (eq? (string? "") #t) "string?")
(ASSERT (eq? (string? 'a) #f) "string?")
(ASSERT (eq? (string? "1") #t) "string?")
(ASSERT (eq? (string? '()) #f) "string?")
(ASSERT (eq? (string? 1234) #f) "string?")
(ASSERT (eq? (string? 1234.5) #f) "string?")
(ASSERT (eq? (string? '(1 2 3)) #f) "string?")


;;
;; number?
;;
(ASSERT (eq? (number? "") #f) "number?")
(ASSERT (eq? (number? 'a) #f) "number?")
(ASSERT (eq? (number? "1") #f) "number?")
(ASSERT (eq? (number? '()) #f) "number?")
(ASSERT (eq? (number? 1234) #t) "number?")
(ASSERT (eq? (number? 1234.5) #t) "number?")
(ASSERT (eq? (number? '(1 2 3)) #f) "number?")

;;
;; float?
;;
(ASSERT (eq? (float? "") #f) "float?")
(ASSERT (eq? (float? 'a) #f) "float?")
(ASSERT (eq? (float? "1") #f) "float?")
(ASSERT (eq? (float? '()) #f) "float?")
(ASSERT (eq? (float? 1234) #f) "float?")
(ASSERT (eq? (float? 1234.5) #t) "float?")
(ASSERT (eq? (float? '(1 2 3)) #f) "float?")

;;
;; integer?
;;
(ASSERT (eq? (integer? "") #f) "integer?")
(ASSERT (eq? (integer? 'a) #f) "integer?")
(ASSERT (eq? (integer? "1") #f) "integer?")
(ASSERT (eq? (integer? '()) #f) "integer?")
(ASSERT (eq? (integer? 1234) #t) "integer?")
(ASSERT (eq? (integer? 1234.5) #f) "integer?")
(ASSERT (eq? (integer? '(1 2 3)) #f) "integer?")

;;
;; function?
;;
(ASSERT (eq? (function? "") #f) "function?")
(ASSERT (eq? (function? '(1 2 3)) #f) "function?")
(ASSERT (eq? (function? +) #t) "function?")
(ASSERT (eq? (function? (lambda () #void)) #t) "function?")

;;
;; macro?
;;
(ASSERT_L #__LINE__ (eq? (macro? "") #f) "macro?")
(ASSERT_L #__LINE__ (eq? (macro? '(1 2 3)) #f) "macro?")
(ASSERT_L #__LINE__ (eq? (macro? +) #f) "macro?")
(ASSERT_L #__LINE__ (eq? (macro? (macro () #void)) #t) "macro?")
(ASSERT_L #__LINE__ (eq? (macro? and) #t) "macro?")

;;
;; array?
;;
(ASSERT_L #__LINE__ (eq? (array? (make-array 0)) #t) "array?")
(ASSERT_L #__LINE__ (eq? (array? (make-array 10)) #t) "array?")
(ASSERT_L #__LINE__ (eq? (array? '()) #f) "array?")
(ASSERT_L #__LINE__ (eq? (array? 1234) #f) "array?")
(ASSERT_L #__LINE__ (eq? (array? +) #f) "array?")
(ASSERT_L #__LINE__ (eq? (array? and) #f) "array?")

;;
;; bytearray?
;;
(ASSERT_L #__LINE__ (eq? (bytearray? (make-bytearray 0)) #t) "bytearray?")
(ASSERT_L #__LINE__ (eq? (bytearray? (make-bytearray 10)) #t) "bytearray?")
(ASSERT_L #__LINE__ (eq? (bytearray? '()) #f) "bytearray?")
(ASSERT_L #__LINE__ (eq? (bytearray? 1234) #f) "bytearray?")
(ASSERT_L #__LINE__ (eq? (bytearray? +) #f) "bytearray?")
(ASSERT_L #__LINE__ (eq? (bytearray? and) #f) "bytearray?")

;;
;; null?
;;
(ASSERT (eq? (null? '()) #t) "null?")
(ASSERT (eq? (null? '(1 2 3)) #f) "null?")

;;
;; char?
;;
(ASSERT (eq? (char? '()) #f) "char?")
(ASSERT (eq? (char? '(1 2 3)) #f) "char?")
(ASSERT (eq? (char? #\h) #t) "char?")

;; ----------------
;; relOps
;; ----------------

;;
;; =
;;
(ASSERT (eq? (= 0 0) #t) "=")
(ASSERT (eq? (= 0 0.0) #t) "=")
(ASSERT (eq? (= 0.0 0) #t) "=")
(ASSERT (eq? (= 0.0 0.0) #t) "=")

(ASSERT (eq? (= 10 10) #t) "=")
(ASSERT (eq? (= 10 10.0) #t) "=")
(ASSERT (eq? (= 10.0 10) #t) "=")
(ASSERT (eq? (= 10.0 10.0) #t) "=")

(ASSERT (eq? (= 10 11) #f) "=")
(ASSERT (eq? (= 10 11.0) #f) "=")
(ASSERT (eq? (= 10.0 11) #f) "=")
(ASSERT (eq? (= 10.0 11.0) #f) "=")

;;
;; <
;;
(ASSERT (eq? (< 10 11) #t) "<")
(ASSERT (eq? (< 10.0 11) #t) "<")
(ASSERT (eq? (< 10 11.0) #t) "<")
(ASSERT (eq? (< 10.0 11.0) #t) "<")

(ASSERT (eq? (< 10 10) #f) "<")
(ASSERT (eq? (< 10.0 10) #f) "<")
(ASSERT (eq? (< 10 10.0) #f) "<")
(ASSERT (eq? (< 10.0 10.0) #f) "<")

(ASSERT (eq? (< 10 9) #f) "<")
(ASSERT (eq? (< 10.0 9) #f) "<")
(ASSERT (eq? (< 10 9.0) #f) "<")
(ASSERT (eq? (< 10.0 9.0) #f) "<")

(ASSERT_L #__LINE__ (eq? (< -2 -1) #t) "<")
(ASSERT_L #__LINE__ (eq? (< -1 0) #t) "<")
(ASSERT_L #__LINE__ (eq? (< -1 1) #t) "<")
(ASSERT_L #__LINE__ (eq? (< 0 1) #t) "<")

(ASSERT_L #__LINE__ (eq? (< -1 -2) #f) "<")
(ASSERT_L #__LINE__ (eq? (< 0 -1) #f) "<")
(ASSERT_L #__LINE__ (eq? (< 1 -1) #f) "<")
(ASSERT_L #__LINE__ (eq? (< 1 0) #f) "<")

;;
;; >
;;
(ASSERT (eq? (> 10 11) #f) ">")
(ASSERT (eq? (> 10.0 11) #f) ">")
(ASSERT (eq? (> 10 11.0) #f) ">")
(ASSERT (eq? (> 10.0 11.0) #f) ">")

(ASSERT (eq? (> 10 10) #f) ">")
(ASSERT (eq? (> 10.0 10) #f) ">")
(ASSERT (eq? (> 10 10.0) #f) ">")
(ASSERT (eq? (> 10.0 10.0) #f) ">")

(ASSERT (eq? (> 10 9) #t) ">")
(ASSERT (eq? (> 10.0 9) #t) ">")
(ASSERT (eq? (> 10 9.0) #t) ">")
(ASSERT (eq? (> 10.0 9.0) #t) ">")

(ASSERT (eq? (> -2 -1) #f) "<")
(ASSERT (eq? (> -1 0) #f) "<")
(ASSERT (eq? (> -1 1) #f) "<")
(ASSERT (eq? (> 0 1) #f) "<")

(ASSERT (eq? (> -1 -2) #t) "<")
(ASSERT (eq? (> 0 -1) #t) "<")
(ASSERT (eq? (> 1 -1) #t) "<")
(ASSERT (eq? (> 1 0) #t) "<")

;;
;; >=
;;
(ASSERT (eq? (>= 10 11) #f) ">=")
(ASSERT (eq? (>= 10.0 11) #f) ">=")
(ASSERT (eq? (>= 10 11.0) #f) ">=")
(ASSERT (eq? (>= 10.0 11.0) #f) ">=")

(ASSERT (eq? (>= 10 10) #t) ">=")
(ASSERT (eq? (>= 10.0 10) #t) ">=")
(ASSERT (eq? (>= 10 10.0) #t) ">=")
(ASSERT (eq? (>= 10.0 10.0) #t) ">=")

(ASSERT (eq? (>= 10 9) #t) ">=")
(ASSERT (eq? (>= 10.0 9) #t) ">=")
(ASSERT (eq? (>= 10 9.0) #t) ">=")
(ASSERT (eq? (>= 10.0 9.0) #t) ">=")

;;
;; <=
;;
(ASSERT (eq? (<= 10 11) #t) "<=")
(ASSERT (eq? (<= 10.0 11) #t) "<=")
(ASSERT (eq? (<= 10 11.0) #t) "<=")
(ASSERT (eq? (<= 10.0 11.0) #t) "<=")

(ASSERT (eq? (<= 10 10) #t) "<=")
(ASSERT (eq? (<= 10.0 10) #t) "<=")
(ASSERT (eq? (<= 10 10.0) #t) "<=")
(ASSERT (eq? (<= 10.0 10.0) #t) "<=")

(ASSERT (eq? (<= 10 9) #f) "<=")
(ASSERT (eq? (<= 10.0 9) #f) "<=")
(ASSERT (eq? (<= 10 9.0) #f) "<=")
(ASSERT (eq? (<= 10.0 9.0) #f) "<=")

;;
;; !=
;;
(ASSERT (eq? (!= 10 11) #t) "!=")
(ASSERT (eq? (!= 10.0 11) #t) "!=")
(ASSERT (eq? (!= 10 11.0) #t) "!=")
(ASSERT (eq? (!= 10.0 11.0) #t) "!=")

(ASSERT (eq? (!= 10 10) #f) "!=")
(ASSERT (eq? (!= 10.0 10) #f) "!=")
(ASSERT (eq? (!= 10 10.0) #f) "!=")
(ASSERT (eq? (!= 10.0 10.0) #f) "!=")

;;
;; ----------------
;; arithmetic functions
;; ----------------

;;
;; +
;;
(ASSERT (= (+) 0) "+")
(ASSERT (= (+ 1) 1) "+")
(ASSERT (= (+ 10 20 30 40) 100) "+")

(ASSERT (integer? (+)) "+")
(ASSERT (integer? (+ 1)) "+")
(ASSERT (integer? (+ 20)) "+")
(ASSERT (integer? (+ 10 20 30 40)) "+")

(ASSERT (= (+ 1.0) 1.0) "+")
(ASSERT (= (+ 10.0 20 30 40) 100.0) "+")
(ASSERT (= (+ 10 20 30 40.0) 100.0) "+")

(ASSERT (float? (+ 1.0)) "+")
(ASSERT (float? (+ 20.0)) "+")
(ASSERT (float? (+ 10.0 20 30 40)) "+")

;;
;; -
;;
(ASSERT (= (- 1) -1) "-")
(ASSERT (= (- 100 10 20 30 40) 0) "-")

(ASSERT (integer? (- 1)) "-")
(ASSERT (integer? (- 20)) "-")
(ASSERT (integer? (- 100 10 20 30 40)) "-")

(ASSERT (= (- 1.0) -1.0) "-")
(ASSERT (= (- 100 10.0 20 30 40) 0.0) "-")
(ASSERT (= (- 100.0 10 20 30 40.0) 0.0) "-")

(ASSERT (float? (- 1.0)) "-")
(ASSERT (float? (- 100 10.0 20 30 40)) "-")
(ASSERT (float? (- 100.0 10.0 20 30 40)) "-")
(ASSERT (float? (- 100.0 10 20 30 40)) "-")

;;
;; *
;;
(ASSERT (= (*) 1) "*")
(ASSERT (= (* 10) 10) "*")
(ASSERT (= (* 10 20) 200) "*")
(ASSERT (= (* 10 20 -2) -400) "*")

(ASSERT (integer? (*)) "*")
(ASSERT (integer? (* 10)) "*")
(ASSERT (integer? (* 10 20)) "*")
(ASSERT (integer? (* 10 20 -2)) "*")

(ASSERT (= (* 1.0) 1.0) "*")
(ASSERT (= (* 10 20.0) 200.0) "*")
(ASSERT (= (* 10.0 20) 200.0) "*")
(ASSERT (= (* 10.0 20.0) 200.0) "*")

(ASSERT (float? (* 1.0)) "*")
(ASSERT (float? (* 10 20.0)) "*")
(ASSERT (float? (* 10.0 20)) "*")
(ASSERT (float? (* 10.0 20.0)) "*")

;;
;; /
;;
(ASSERT (= (/ 1) 1) "/")
(ASSERT (= (/ 10 2) 5) "/")
(ASSERT (= (/ 20 2 2) 5) "/")
(ASSERT (= (/ 100 10 2) 5) "/")

(ASSERT (= (/ 1.0) 1.0) "/")
(ASSERT (= (/ 10 2.0) 5.0) "/")
(ASSERT (= (/ 20.0 2) 10.0) "/")
(ASSERT (= (/ 20.0 2.0) 10.0) "/")


;; ----------------
;; string functions
;; ----------------

;;
;; string-length
;;
(ASSERT (= (string-length "") 0) "string-length")
(ASSERT (= (string-length "1") 1) "string-length")
(ASSERT (= (string-length "1234") 4) "string-length")

;;
;; string=?
;;
(ASSERT (eq? (string=? "" "") #t) "string=?")
(ASSERT (eq? (string=? "" "aa") #f) "string=?")
(ASSERT (eq? (string=? "1234" "1234") #t) "string=?")

;;
;; string-append
;;
(ASSERT (eq? (string-length (string-append )) 0) "string-append")
(ASSERT (eq? (string-length (string-append "")) 0) "string-append")
(ASSERT (eq? (string-length (string-append "a")) 1) "string-append")
(ASSERT (eq? (string-length (string-append "" "a")) 1) "string-append")
(ASSERT (eq? (string-length (string-append "" "a" "")) 1) "string-append")
(ASSERT (eq? (string-length (string-append "a" "")) 1) "string-append")
(ASSERT (eq? (string-length (string-append "a" "bb" "ccc")) 6) "string-append")

;; ----------------
;; list functions
;; ----------------

;;
;; list
;;
(ASSERT (equal? (list) '()) "list")
(ASSERT (equal? (list 1) '(1)) "list")
(ASSERT (equal? (list 1 2 3) '(1 2 3)) "list")

;;
;; length
;;
(ASSERT (= (length '()) 0) "length")
(ASSERT (= (length '(1 2 3)) 3) "length")

;;
;; append
;;
(ASSERT (equal? (append '() '()) '()) "append")
(ASSERT (equal? (append '(1 2 3) '()) '(1 2 3)) "append")
(ASSERT (equal? (append '() '(1 2 3)) '(1 2 3)) "append")
(ASSERT (equal? (append '() '(1 2 3) ) '(1 2 3)) "append")
(ASSERT (equal? (append '(1 2 3) '(4 5 6) ) '(1 2 3 4 5 6)) "append")

;;
;; reverse
;;
(ASSERT (equal? (reverse '()) '()) "reverse")
(ASSERT (equal? (reverse '(1)) '(1)) "reverse")
(ASSERT (equal? (reverse '(1 2 3)) '(3 2 1)) "reverse")

;
; map
;
(ASSERT (equal? (map (lambda (n) (* n 2)) '()) '()) "map")
(ASSERT (equal? (map (lambda (n) (* n 2)) '(1)) '(2)) "map")
(ASSERT (equal? (map (lambda (n) (* n 2)) '(1 2 3 4)) '(2 4 6 8)) "map")

;
; select
;
(ASSERT (equal? (select (lambda (n) (> n 5)) '()) '()) "select")
(ASSERT (equal? (select (lambda (n) (> n 5)) '(1 2 3 1 2 3)) '()) "select")
(ASSERT (equal? (select (lambda (n) (> n 5)) '(1 6 2 7 3 8 9)) '(6 7 8 9)) "select")

;
; find-first
;
(ASSERT (equal? (find-first (lambda (n) (> n 5)) '()) nil) "find-first")
(ASSERT (equal? (find-first (lambda (n) (> n 5)) '(1 2 3 1 2 3)) nil) "find-first")
(ASSERT (equal? (find-first (lambda (n) (> n 5)) '(1 6 2 7 3 8 9)) 1) "find-first")
(ASSERT (equal? (find-first (lambda (n) (> n 7)) '(1 6 2 7 3 8 9)) 5) "find-first")
(ASSERT (equal? (find-first (lambda (n) (> n 0)) '(1 6 2 7 3 8 9)) 0) "find-first")


;
; flatten
;
(ASSERT (equal? (flatten '()) '()) "flatten")
(ASSERT (equal? (flatten '(1)) '(1)) "flatten")
(ASSERT (equal? (flatten '( (1) )) '(1)) "flatten")
(ASSERT (equal? (flatten '( 1 2 )) '(1 2)) "flatten")
(ASSERT (equal? (flatten '( (1) 2 )) '(1 2)) "flatten")
(ASSERT (equal? (flatten '( 1 (2) )) '(1 2)) "flatten")
(ASSERT (equal? (flatten '( (1) (2) )) '(1 2)) "flatten")
(ASSERT (equal? (flatten '(1 2 3 4 (5 6 7) 8 (9 ) 10 (11 12))) '(1 2 3 4 5 6 7 8 9 10 11 12)) "flatten")

;; ----------------
;; character functions
;; ----------------
(ASSERT_L #__LINE__ (char=? #\h #\h) "char=?")
(ASSERT_L #__LINE__ (not (char=? #\h #\H)) "char=?")
(ASSERT_L #__LINE__ (= (char->integer #\h) 104) "char->integer")
(ASSERT_L #__LINE__ (char=? (integer->char 104) #\h) "integer->char")

;; ----------------
;; string functions
;; ----------------
(ASSERT_L #__LINE__ (string=? "hello" "hello") "string=?")
(ASSERT_L #__LINE__ (not (string=? "hello" "Hello")) "string=?")
(ASSERT_L #__LINE__ (string=? (string-append "hello" "") "hello") "string-append")
(ASSERT_L #__LINE__ (string=? (string-append "x" "hello" "x") "xhellox") "string-append")
(ASSERT_L #__LINE__ (char=? (string-ref "hello" 0) #\h) "string-ref")
((lambda ()
    (define s (string-append "hello" ""))  ; make a mutable copy
    (string-set! s 0 #\H)
    (ASSERT_L #__LINE__ (eq? (string-ref "hello" 0) #\h) "string-set!")
))

;; ----------------
;; array functions
;; ----------------
(ASSERT_L #__LINE__ (eq? (array-length (make-array 0)) 0) "array-length")
(ASSERT_L #__LINE__ (eq? (array-length (make-array 10)) 10) "array-length")

(ASSERT_L #__LINE__ (eq? (array-length (make-bytearray 0)) 0) "array-length")
(ASSERT_L #__LINE__ (eq? (array-length (make-bytearray 10)) 10) "array-length")

((lambda ()
    (define arr (make-array 10))
    (ASSERT_L #__LINE__ (eq? (array-length arr) 10) "array-length")
    (ASSERT_L #__LINE__ (eq? (array-ref arr 0) nil) "array-ref")
    (ASSERT_L #__LINE__ (eq? (array-ref arr 9) nil) "array-ref")
    (array-set! arr 0 100)
    (array-set! arr 9 109)
    (ASSERT_L #__LINE__ (eq? (array-ref arr 0) 100) "array-set!")
    (ASSERT_L #__LINE__ (eq? (array-ref arr 9) 109) "array-set!")
))

((lambda ()
    (define arr (make-bytearray 10))
    (ASSERT_L #__LINE__ (eq? (array-length arr) 10) "bytearray-length")
    (ASSERT_L #__LINE__ (eq? (bytearray-ref arr 0) 0) "bytearray-ref")
    (ASSERT_L #__LINE__ (eq? (bytearray-ref arr 9) 0) "bytearray-ref")
    (bytearray-set! arr 0 100)
    (bytearray-set! arr 9 109)
    (ASSERT_L #__LINE__ (eq? (bytearray-ref arr 0) 100) "bytearray-set!")
    (ASSERT_L #__LINE__ (eq? (bytearray-ref arr 9) 109) "bytearray-set!")
))

;; ----------------
;; environment functions
;; ----------------

((lambda ()
    (define env (make-environment))
    (environment-define env 'var1 1234)
    (environment-define env 'var2 2345)
    (environment-define env 'var3 3456)
    (environment-define env 'var4 4567)

    (ASSERT_L #__LINE__ (eq? (environment-ref env 'var3) 3456) "environment-ref")
    (environment-set! env 'var3 9999)
    (ASSERT_L #__LINE__ (eq? (environment-ref env 'var3) 9999) "environment-set!")
))

;; ----------------
;; misc functions
;; ----------------

;;
;; eval
;;
(ASSERT (equal? (eval 123) 123) "eval")
(ASSERT (equal? (eval '(+ 1 2 3)) 6) "eval")
(define global 123)
(ASSERT (equal? (eval 'global) 123) "eval")

; eval
; in a special environment (with a different binding for plus)
; DEMO how debug-helpers could be implemented
; for example:
;   trap on call of a particular function
;   trap if a particular argument value is passed
;   trap on a particular return value
;   trap after being called for N times
;   trace and debug logging
;   instrumentation: count how often a function is called
;   instrumentation: measure the execution time of a function

((lambda ()
    (define special-env (make-environment))
    (environment-define special-env
	'+
	(lambda args
	    (define rslt nil)
	    (display "TRACE: enter plus: " args "\n")
	    (set! rslt (apply + args))
	    (display "TRACE: result: " rslt "\n")
	    rslt
	))

    ; eval in there
    (eval '(+ 1 2 (+ 3 4) 5) special-env)
))

;; ----------------
;; vararg functions
;; ----------------
(define (trest a . b)
    (display "a=" a " b=" b "\n"))

(define (trest1a a . b)
    a)

(define (trest1b a . b)
    b)

(ASSERT (equal? (trest1a 1) 1) "trest1a")
(ASSERT (equal? (trest1a 1 2) 1) "trest1a")
(ASSERT (equal? (trest1a 1 2 3) 1) "trest1a")
(ASSERT (equal? (trest1a 'a 'b 'c) 'a) "trest1a")
(ASSERT (equal? (trest1a 'a 'b 'c 'd) 'a) "trest1a")

(ASSERT (equal? (trest1b 1) '()) "trest1b")
(ASSERT (equal? (trest1b 1 2) '(2)) "trest1b")
(ASSERT (equal? (trest1b 1 2 3) '(2 3)) "trest1b")
(ASSERT (equal? (trest1b 'a 'b 'c) '(b c)) "trest1b")
(ASSERT (equal? (trest1b 'a 'b 'c 'd) '(b c d)) "trest1b")

(define (trest2 . x)
    x)

(ASSERT (equal? (trest2) '()) "trest2")
(ASSERT (equal? (trest2 1) '(1)) "trest2")
(ASSERT (equal? (trest2 1 2) '(1 2)) "trest2")
(ASSERT (equal? (trest2 'a 'b 'c 'd) '(a b c d)) "trest2")

;; ----------------
;; macros
;; ----------------

(define t-and (macro (a b) (list 'if a b #f)))
(define (test a)
    (t-and (> a 0) (<= a 10)))

(ASSERT (eq? (test 5) #t) "t-and")
(ASSERT (eq? (test 10) #t) "t-and")
(ASSERT (eq? (test 0) #f) "t-and")
(ASSERT (eq? (test 11) #f) "t-and")

;; ----------------
;; input-output
;; ----------------
(define s (open-output-file "testfile"))
(display "hello\n" s)
(close s)

(define s (open-input-file "testfile"))
(define line (read-line s))
(close s)
(ASSERT (string=? line "hello") "read-line from fileStream")

(define s (open-output-file "testfile"))
(print '(a b c d "someString" 123 456.789) s)
(close s)

(define s (open-input-file "testfile"))
(define obj (read s))
(close s)
(ASSERT (equal? obj '(a b c d "someString" 123 456.789)) "read")


(define s (open-output-string))
(display "hello" s)
(define line (get-output-string s))
(ASSERT (string=? line "hello") "read-line from stringStream")

;; ----------------
;; macros defined in syntax.scm
;; ----------------

(define (test-and a b)
    (and (> a 0) (<= a 10) (> b 10) (<= b 20)))

(ASSERT (test-and 5 15) "test-and")

(define (test-or a b)
    (or (= a 0) (= a 2) (> b 10) (< b 0)))

(ASSERT (test-or -1 15) "test-or1")
(ASSERT (test-or 0 15) "test-or2")
(ASSERT (test-or 2 15) "test-or3")
(ASSERT (test-or 1 15) "test-or4")
(ASSERT (test-or 1 -1) "test-or5")
(ASSERT (not (test-or 1 1)) "test-or6")

(define (test-let a b)
    (let ((tA (+ a 1)) (tB (- b 2)))
	(+ tA tB)))

(ASSERT (= (test-let 10 20) 29) "test-let")


(define (test-cond x)
    (cond
	((= x 10) 11)
	((= x 20) 21)
	((= x 30) 31)
	(#t 99)))

(ASSERT (= (test-cond 10) 11) "test-cond")
(ASSERT (= (test-cond 20) 21) "test-cond")
(ASSERT (= (test-cond 30) 31) "test-cond")
(ASSERT (= (test-cond 40) 99) "test-cond")

;; ----------------
;; catch & throw
;; ----------------

((lambda ()
    (define funcCalled #f)
    (define handlerCalled #f)
    (define retVal #f)
    (define arrivedAfterThrow #f)

    ; to throw from a deeply nested call
    (define (nestedThrower exception)
	(define (evenDeeper)
	    (define (aliceInWonderlandAtBottom)
		(display "now throw\n")
		(throw exception)
		'valueToPreventTailCall
	    )
	    (aliceInWonderlandAtBottom)
	    'valueToPreventTailCall
	)
	(evenDeeper)
	'valueToPreventTailCall
    )
    (display "---- catch test 1\n")

    (set! retVal
	(catch 'foo
	    (lambda ()
		(display "in func\n")
		(set! funcCalled #t)
		'valueFromFunc
	    )
	    (lambda ()
		(set! handlerCalled #t)
		(display "handler called\n")
		'valueFromHandler
	    )
	)
    )

    (ASSERT_L #__LINE__ funcCalled "function not called")
    (ASSERT_L #__LINE__ (not handlerCalled) "oops - handler should not be called")
    (ASSERT_L #__LINE__ (eq? retVal 'valueFromFunc) "oops - wrong return value")

    (display "---- catch test 2\n")

    (set! funcCalled #f)
    (set! handlerCalled #f)
    (set! arrivedAfterThrow #f)

    (set! retVal
	(catch 'foo
	    (lambda ()
		(set! funcCalled #t)
		(display "in func\n")
		(display "now throw\n")
		(throw 'foo)
		(set! arrivedAfterThrow #t)
		(display "oops - should not be here\n")
		'valueFromFunc
	    )
	    (lambda ()
		(set! handlerCalled #t)
		(display "handler called\n")
		'valueFromHandler
	    )
	)
    )

    (ASSERT_L #__LINE__ funcCalled "function not called")
    (ASSERT_L #__LINE__ handlerCalled "handler not called")
    (ASSERT_L #__LINE__ (not arrivedAfterThrow) "oops - code after throw should not be executed")
    (ASSERT_L #__LINE__ (eq? retVal 'valueFromHandler) "oops - wrong return value")

    (display "---- catch test 3\n")

    (set! funcCalled #f)
    (set! handlerCalled #f)
    (set! arrivedAfterThrow #f)

    (set! retVal
	(catch 'foo
	    (lambda ()
		(set! funcCalled #t)
		(display "in func\n")
		(nestedThrower 'foo)
		(set! arrivedAfterThrow #t)
		(display "oops - should not be here\n")
		'valueFromFunc
	    )
	    (lambda ()
		(set! handlerCalled #t)
		(display "handler called\n")
		'valueFromHandler
	    )
	)
    )

    (ASSERT_L #__LINE__ funcCalled "function not called")
    (ASSERT_L #__LINE__ handlerCalled "handler not called")
    (ASSERT_L #__LINE__ (not arrivedAfterThrow) "oops - code after throw should not be executed")
    (ASSERT_L #__LINE__ (eq? retVal 'valueFromHandler) "oops - wrong return value")
))

(display "\n" "self tests passed\n")
(display "**************************** end of selftest.scm **************\n")
