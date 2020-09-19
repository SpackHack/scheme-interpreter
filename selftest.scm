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

(define nameOrUnnamedIfNil 
    (lambda 
        (arg)
        (if 
            (eq? arg #n)
            "unnamed"
            arg
        )
    )
)

(define ASSERT_L  
    (lambda (lineNr boolean nameOfTest)
        (if 
            boolean
            #v
	        (begin
	            (display (nameOrUnnamedIfNil nameOfTest) ": **** FAILED [" lineNr "]")
	            (abort)
	        )
        )
    )
)

(define ASSERT 
    (lambda 
        (boolean nameOfTest)
        (ASSERT_L "?" boolean nameOfTest)
    )
)

(define ASSERT_RSLT (lambda ( function expected)
    (define result (function))


    (if (equal? result expected)
	(begin
	    (display (nameOrUnnamedIfNil (function-name function)))
	    (display ": ok\n")
	)
	(begin
	    (display (nameOrUnnamedIfNil (function-name function)))
	    (display ": **** FAILED (expected:" expected " got:" result "\n")
	)
    )
))


(display "**************************** start of tests **************\n")

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
;; null?
;;
(ASSERT (eq? (null? '()) #t) "null?")
(ASSERT (eq? (null? '(1 2 3)) #f) "null?")

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

(ASSERT_L 1 (eq? (< -2 -1) #t) "<")
(ASSERT_L 2 (eq? (< -1 0) #t) "<")
(ASSERT_L 3 (eq? (< -1 1) #t) "<")
(ASSERT_L 4 (eq? (< 0 1) #t) "<")

(ASSERT_L 5 (eq? (< -1 -2) #f) "<")
(ASSERT_L 6 (eq? (< 0 -1) #f) "<")
(ASSERT_L 7 (eq? (< 1 -1) #f) "<")
(ASSERT_L 8 (eq? (< 1 0) #f) "<")

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

(display "self tests passed")
(display "**************************** end of selftest.scm **************\n")
