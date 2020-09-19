(display "Guess a Number Game")
(define i (open ""))

(display "Enter max Number")
(define max (read-line i))

(define targetNumber (random max))
(define numberOfGuesses 0)


(define testNumber (lambda () 
    (display "Guess a Number")
    (define guess (read-line i))
    (set numberOfGuesses (+ numberOfGuesses 1))
    (if (= guess targetNumber)
        #v
        (begin 
            (if (> guess targetNumber)
                (display "Not the Number !! The number is lower") 
                (display "Not the Number !! The number is higher") 
            )
            (testNumber)    
        )
    )
))
(testNumber)
(close i)
(display "You Finished the Game with " numberOfGuesses " Guesses")
