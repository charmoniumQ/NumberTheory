(module triangle_chart racket/base
	(require "racket_specific.rkt")
	(require "lisp_utils.rkt")
	(require "color.rkt")
	(require srfi/1)
	(require racket/file)
	(require 2htdp/image)

	(define (stretch val e max-level) (expt (/ val max-level) e))

	(define (get-color val e max-level)
		(if (= e 0)
			(make-color 50 50 50)
			(viridis (stretch val e max-level))))

	(define (make-cell f render? e level max-level sq-size text-size n)
			(let* (
				[val (f n level)]
				[color (get-color val e max-level)]
				[rect-contents (rectangle sq-size sq-size "solid" color)])
				(if (render? val n level)
					(if (eq? text-size 0)
						rect-contents
						(let ([text-contents (text (format "~a" val) text-size (viridis-foreground (stretch val e max-level)))])
						  (overlay text-contents rect-contents)))
					(rectangle sq-size sq-size "solid" "white"))))

	(define (make-row f render? e max-level sq-size text-size level)
		(let* (
			[padding-length (- max-level level)]
			[half-padding (rectangle (/ sq-size 2) sq-size "solid" "white")]
			[padding (make-list padding-length half-padding)]
			[my-make-cell (curry make-cell f render? e level max-level sq-size text-size)]
			[cells (map my-make-cell (rangei 0 level))])
		(apply beside (append padding cells padding))))

	(define (make-grid f render? e max-level sq-size text-size)
		(let* (
			[my-make-row (curry make-row f render? e max-level sq-size text-size)]
			[rows (map my-make-row (rangei 0 max-level))])
			(apply above rows)))

	(define (save-grid f render? e max-level sq-size text-size fname)
		(display (format "generating: ~a~n" fname))
		(save-image (make-grid f render? e max-level sq-size text-size) fname))

	(provide make-grid save-grid))
