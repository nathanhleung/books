#lang racket


; 1.1.1
420
(+ 400 20)
(- 500 80)
(* 20 21)
(/ 8400 20)
(+ 400 19 1)
(+ 1 (+ 1 1))

; 1.1.2
(define size 2)
size
(* 210 size)

; 1.1.4
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