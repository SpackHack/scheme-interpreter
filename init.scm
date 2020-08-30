(define a 1)
(define func (lambda (a) (print-env) (define a 5) (print-env) a))
(define b (lambda (a) a))
(define func1 (lambda () a))