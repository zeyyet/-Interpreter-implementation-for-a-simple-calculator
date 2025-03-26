calculator(Expression, Result) :-
    (   Expression = _ / 0
    ->  write('hata: 0a bölünme'), nl, fail
    ;   Result is Expression
    ).
