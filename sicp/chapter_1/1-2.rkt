#lang racket

(require "lib.rkt")

; 1.2.1
(display_chapter_heading "1.2.1")
(define (factorial n)
  (if (= n 1)
      1
      (* n (factorial (- n 1)))))
(factorial 6)

(define (factorial-iter n)
  (fact-iter 1 1 n))

(define (fact-iter product counter max-count)
  (if (> counter max-count)
      product
      (fact-iter (* product counter) (+ counter 1) max-count)))
(factorial-iter 6)

; Exercise 1.9
(define (inc a) (+ a 1))
(define (dec a) (+ a (- 1)))

(+ 4 5)
(inc (+ (dec 4) 5))
(inc (+ 3 5))
(inc (inc (+ (dec 3) 5)))
(inc (inc (+ 2 5)))
(inc (inc (inc (+ (dec 2) 5))))
(inc (inc (inc (+ 1 5))))
(inc (inc (inc (inc (+ (dec 1) 5)))))
(inc (inc (inc (inc (+ 0 5)))))
(inc (inc (inc (inc 5))))
; This implementation of + is recursive

(+ 4 5)
(+ (dec 4) (inc 5))
(+ 3 6)
(+ (dec 3) (inc 6))
(+ 2 7)
(+ (dec 2) (inc 7))
(+ 1 8)
(+ (dec 1) (inc 8))
(+ 0 9)
; This implementation of + is iterative

(define (A x y)
  (cond ((= y 0) 0)
        ((= x 0) (* 2 y))
        ((= y 1) 2)
        (else (A (- x 1) (A x (- y 1))))))

; Exercise 1.10
(A 1 10)
(A (- 1 1) (A 1 (- 10 1)))
(A 0 (A 1 9))
(A 0 (A 0 (A 1 8)))
; ... (A 0 (A 0 (A 0 ... (A 1 1))))
; ; ... (A 0 (A 0 (A 0 ... 2)))
; 2^10

(A 2 4)
(A (- 2 1) (A 2 (- 4 1)))
(A 1 (A 2 3)) ; From above, we know this is 2^(A 2 3)
(A 1 (A 1 (A 2 2)))
(A 1 (A 1 (A 1 (A 2 1))))
(A 1 (A 1 (A 1 2)))
(A 1 (A 1 (A (- 1 1) (A 1 (- 2 1)))))
(A 1 (A 1 (A 0 (A 1 1))))
(A 1 (A 1 (A 0 2)))
(A 1 (A 1 4)) ; From above, we know this is 2^(2^4) = 2^16

; Pattern for (A x y) = (A (- x 1) (A (- x 1) (A (- x 1) ... (A (- x 1) (A (- x 1) 1)) ... )))
;                     = (A (- x 1) (A (- x 1) (A (- x 1) ... (A (- x 1) 2) ... )))
; (x - 1) appears (x - 1) times
; So, (A 1 (A 1 (A 1 ... ))) = 2^(2^(2^(2 ... 2^2)))

; (A 1) n - 1 times
; (A 2 n) = (A 1 (A 1 (A 1 ... (A 1 (A 2 1)) ... )
;         = (A 1 (A 1 (A 1 ... (A 1 2) ... ) with n - 1 1s
; Recall (A 1 2) = 2^2
; So we have (A 1 (A 1 (A 1 ... (A 1 (A 1 4)))))) with n - 2 1s
; Recursively expanding, we have (A 1 (A 1 (A 1 ... (A 1 16))))) with n - 3 1s
; So the expression is 2^2^2^...^2^16 with n-3 2s
; which becomes 2^(2^)

(A 3 3)
(A (- 3 1) (A 3 (- 3 1)))
(A 2 (A 3 2))
(A 2 (A (- 3 1) (A 3 (- 2 1))))
(A 2 (A 2 (A 3 1)))
(A 2 (A 2 2))
; From above, (A 2 2) = (A 1 (A 2 1)) = (A 1 2) = 2^2
(A 2 4)

(define (f n) (A 0 n)) ; 2n = 2 * (A 0 (- 1 n))
(f 1); 2
(f 2); 2 * 2
(f 3); 2 * 3
(f 4); 2 * 4

(define (g n) (A 1 n)) ; 2^n = 2 * (A 1 (- 1 n))
(g 1); 2^1
(g 2); 2^2
(g 3); 2^3
(g 4); 2^4

(define (h n) (A 2 n)) ; 2^2^2^2 (n times) = 2 ^ (A 2 (- 2 n))
(h 1); 2^1 = 2
(h 2); 2^2 = 2^2
(h 3); 2^4 = 2^2^2
(h 4); 2^16 = 2^2^2^2
