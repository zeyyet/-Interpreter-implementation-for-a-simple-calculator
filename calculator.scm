(define (calculator expr)
  (cond ((number? expr) expr)  ; eğer ifade bir sayıysa sayıyı direkt döndür
        ((list? expr)  ; eğer ifade bir listeyse işlem yapılacak
         (let ((op (car expr))  ; listenin ilk elemanı işlemolur
               (args (cdr expr)))  ; geri kalan kısım argümanlar olur
           (case op  ; işleme göre farklı bir işlem yap
             ((+) (+ (calculator (car args)) (calculator (cadr args))))
             ((-) (- (calculator (car args)) (calculator (cadr args))))
             ((*) (* (calculator (car args)) (calculator (cadr args))))
             ((/)(let ((den (calculator (cadr args))))  ; payda
                (if (= den 0)
                    (begin (display "hata 0a bölünme\n") 0)
                    (/ (calculator (car args)) den))))
             (else (begin (display "hata\n") 0))))))) 
