(define not (lambda (bool) (if bool #f #t)))
(define < (lambda (a b) (> b a)))
(define >= (lambda (a b) (not (< a b))))
(define <= (lambda (a b) (not (> a b))))

(define func 
    (lambda
        (a)
        (print a)
        (func (+ a 1))
    )
)

(if #t 1 2)
(if #f 1 2)

(define for-loop (lambda (start stop fn)
    (if (> start stop)
       #v
    ; else
       (begin (fn start) (for-loop (+ 1 start) stop fn))
    )
))

(display "done with init.scm")
