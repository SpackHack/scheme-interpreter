(define not (lambda (bool) (if bool #f #t)))
(define < (lambda (a b) (> b a)))
(define >= (lambda (a b) (not (< a b))))
(define <= (lambda (a b) (not (> a b))))

(define for-loop (lambda (start stop fn)
    (if (> start stop)
       #v
       (begin (fn start) (for-loop (+ 1 start) stop fn))
    )
))

(define sum-to (lambda (n)
    (define sum 0)
    (for-loop 1 n (lambda (i) (print i) (set sum (+ sum i))))))


(define reload (lambda () (load "init.scm")))
(display "done with init.scm")
