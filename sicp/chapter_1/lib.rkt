#lang racket

(provide display_chapter_heading)

(define (display_chapter_heading chapter)
  (display "===")
  (display chapter)
  (display "===")
  (newline))