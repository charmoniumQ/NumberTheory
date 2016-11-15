(module color racket/base
	(require "racket_specific.rkt")
	(require "lisp_utils.rkt")
	(require 2htdp/image)
	(require racket/file)
	(module+ test (require rackunit))

	(define viridis-data (let* (
		[filename "viridis.csv"]
		[str-contents (file->string "viridis.csv")]
		[list-contents (from-csv str-contents)]
		[num-contents (nested-map string->number list-contents 2)]
		[vec-contents (list->vector (map list->vector num-contents))])
		vec-contents))

	(define (viridis val) (let* (
		[n (- (vector-length viridis-data) 1)]
		[i (exact-floor (* val n))]
		[color-val (vector->list (vector-ref viridis-data i))]
		[color-int (map (lambda (v) (exact-round (* v 255))) color-val)])
		(apply make-color color-int)))
	(module+ test (viridis 0.3))

	(define black (make-color 0 0 0))
	(define white (make-color 255 255 255))

	(define (viridis-foreground val)
		(if (< val 0.4) white black))

	(provide viridis viridis-foreground))
