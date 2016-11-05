(module pascal racket/base
	(require memoize)
	(require srfi/1)
	(require "racket_specific.rkt")
	(require "lisp_utils.rkt")
	(require "triangle_chart.rkt")
	(require "characteristic.rkt")

	(define/memo (pascal-row n)
		(if (= n 0)
			'(1)
			(map +
				(append '(0) (pascal-row (- n 1)))
				(append (pascal-row (- n 1)) '(0)))))

	(define (pascal n) (map pascal-row (range 0 n)))

	(define (pascal-mod n) (map (curryr weird-mod n) (pascal n)))

(define (f a b) (abs (- (k-characteristic a b) (weird-mod (list-ref (pascal-row b) a) b))))
	(module+ main
		
		(define max-level 50)
		(define sq-size 20)
		(define text-size 8)
		(define coloration 0.5)
		(save-grid f (lambda (val n level) #t) coloration max-level sq-size text-size "output/pascal.png")
	)

	(provide pascal-row pascal f))
