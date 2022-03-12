#lang racket

(require "lib.rkt")

; 1.1.1
(display_chapter_heading "1.1.1")
420
(+ 400 20)
(- 500 80)
(* 20 21)
(/ 8400 20)
(+ 400 19 1)
(+ 1 (+ 1 1))

; 1.1.2
(display_chapter_heading "1.1.2")
(define size 2)
size
(* 210 size)

; 1.1.4
(display_chapter_heading "1.1.4")
(define (square x) (* x x))
(square 20)
(define (cube x) (* x (square x)))
(cube 9)
(define (tesseract x) (* (square x) (square x)))
(tesseract 11)
(define (tesseract2 x) (square (square x)))
(tesseract2 11)
(define (fifth_power x) (* (square x) (cube x)))
(fifth_power 11)

; 1.1.6
(display_chapter_heading "1.1.6")
(define (abs x)
  (cond ((> x 0) x)
        ((= x 0) 0)
        ((< x 0) (- x))))

(abs -1)
(abs 0)
(abs 1)

(define (abs2 x)
  (cond ((< x 0) (- x))
        (else x)))

(abs2 -2)
(abs2 0)
(abs2 2)

(define (abs3 x)
  (if (< x 0)
      (- x)
      x))

(define (gte x y)
  (or (> x y) (= x y)))

(gte 1 2)
(gte 1 1)
(gte 1 0)

(define (gte2 x y)
  (not (< x y)))

(gte2 1 2)
(gte 1 1)
(gte 1 0)

; Exercise 1.2
(/ (+ 5 4 (- 2 (- 3 (+ 6 (/ 4 5))))) (* 3 (- 6 2) (- 2 7)))

; Exercise 1.3
(define (sum_of_squares_of_larger a b c)
  (define minimum (min a b c))
  ; Whatever number is equal to the minimum, get the sum of the other two.
  ; This correctly handles the case when any of a = b, a = c, or b = c.
  (cond ((= minimum a) (+ (square b) (square c)))
        ((= minimum b) (+ (square a) (square c)))
        ((= minimum c) (+ (square a) (square b)))))

(sum_of_squares_of_larger 1 2 3)
(sum_of_squares_of_larger 1 3 2)
(sum_of_squares_of_larger 2 3 1)
(sum_of_squares_of_larger 2 1 3)
(sum_of_squares_of_larger 3 1 2)
(sum_of_squares_of_larger 3 2 1)

; Exercise 1.5
(define (p) (p))
(define (q) 10)

(define (test x y)
  (if (= x 0)
      0
      y))

; Applicative order will recurse infinitely while trying to evaluate `(p)`
; Normal order will expand out the call to `test`, see that `(= x 0)` is `#t`,
; and resolve to 0
; (test 0 (p))

; 1.1.7
(display_chapter_heading "1.1.7")

(define (sqrt-iter guess x)
  (if (good-enough? guess x)
      guess
      (sqrt-iter (improve guess x) x)))

(define (improve guess x)
  (average guess (/ x guess)))

(define (average x y)
  (/ (+ x y) 2))

(define (good-enough? guess x)
  (< (abs (- (square guess) x)) 0.001))

(define (sqrt x)
  (sqrt-iter 1.0 x))

(sqrt 9)
(sqrt (+ 100 37))
(sqrt (+ (sqrt 2) (sqrt 3)))
(square (sqrt 1000))

; Exercise 1.6
(define (new-if predicate then-clause else-clause)
  (cond (predicate then-clause)
        (else else-clause)))

(new-if (= 2 3) 0 5)

(define (new-sqrt-iter guess x)
  (new-if (good-enough? guess x)
          guess
          (new-sqrt-iter (improve guess x) x)))

(define (new-sqrt x)
  (new-sqrt-iter 1.0 x))

; The special short-circuit processing rule for `if` doesn't apply to `new-if`,
; so since the interpreter uses applicative-order evaluation, it will hang as it
; recursively tries to evaluate the call to `sqrt-iter`.
; (new-sqrt 10)

; Exercise 1.7
(sqrt (square 0.01)) ; Epsilon is too large for this to be accurate
(sqrt (square 9))
(sqrt (square 123456789123456789123456789)) ; This seems pretty close

(define (better-sqrt x)
  (better-sqrt-iter 1.0 x))

(define (better-sqrt-iter guess x)
  (define next-guess (improve guess x))
  (if (better-good-enough? guess next-guess)
      next-guess
      (better-sqrt-iter next-guess x)))

(define (better-good-enough? previous-guess guess)
  (< (/ (abs (- guess previous-guess))
        previous-guess)
     ; End when change in guess is less than 0.1%
     0.001))

(better-sqrt (square 0.01))
(better-sqrt (square 9))
(better-sqrt (square 123456789123456789123456789))

; On a percent error basis, this new function `better-sqrt` performs
; better on small numbers and worse on large numbers (probably since
; epsilon was so small relative to the large numbers in the previous
; version of the function).

; Exercise 1.8
(define (cbrt-iter guess x)
  (define next-guess (improve-cbrt guess x))
  (if (better-good-enough? guess next-guess)
      next-guess
      (cbrt-iter next-guess x)))

(define (improve-cbrt guess x)
  (average guess (/ (+ (/ x (square guess)) (* 2 guess)) 3)))

(define (cbrt x)
  (cbrt-iter 1.0 x))

(cbrt (cube 0.01))
(cbrt (cube 9))
(cbrt (cube 123456789123456789123456789))
