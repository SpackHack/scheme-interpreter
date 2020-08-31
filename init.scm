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

(display "done with init.scm")