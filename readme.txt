Zeynep Yetkin
231101042


Rust:
calculator.rs

Uygulama Açıklaması: girdi ifadesindeki tüm whitespaceleri kaldırarak rekürsif iniş analizi ile ifadeyi parçalara ayırır. Aşağıdaki özellikleri içerir:

İfadeler: Toplama, çıkarma, çarpma, bölme, parantez içi ifadeler.
Değişken Ataması: Örneğin x=5 veya x=(3+4)*2 gibi ifadeleri işler.
geçersiz karakter, eksik parantez veya sıfıra bölme durumunda hata mesajı verir.

Çalıştırması:

rustc calculator.rs
./calculator

Kullanım: 
bir ifade girin: (çıkmak için 'exit')  (3+4)*2
sonuç: 14
Çıkmak için exit yazın.


Ada:
calculator.adb

Uygulama Açıklaması: kullanıcıdan sırasıyla iki sayı ve bir operatör alarak temel aritmetik işlemleri yapar. Parantezli işlemleri veya ikiden çok sayılı işlemleri yapamaz.

İşlemler: Toplama, çıkarma, çarpma, bölme.
Hata Yönetimi: Bölme sırasında payda 0 ise hata mesajı gösterir, geçersiz operatör durumunda bilgi verir.

Çalıştırılması:

gnatmake calculator.adb
./calculator

Kullanım:
bir ifade girin:
5
+
3
sonuç: 8


Perl:
calculator.pl

Uygulama Açıklaması: girdi ifadesinde yer alan değişken isimlerini kontrol eder, eğer daha önceden tanımlanmışsa değerlerini yerine koyar.

Değişken Ataması: x=5 gibi ifadelerle değişken ataması yapar.
İfade Hesaplama: eval fonksiyonu kullanılarak ifadeyi hesaplar.
Hata Yönetimi: Geçersiz ifadeler veya sıfıra bölme durumunda hata mesajı verir.

Çalıştırılması:

perl calculator.pl

Kullanım: 
bir ifade girin: (çıkmak için 'exit')  (3+4)*2
sonuç: 14
Çıkmak için exit yazın.


Scheme:
calculator.scm

Uygulama Açıklaması: liste yapısında girilen aritmetik ifadeleri değerlendirir.

İfadeler: Sayılar ve temel aritmetik işlemler içeren listeler şeklinde ifade edilir. Örneğin:
(calculator '(+ 3 4))
Hata Yönetimi: Sıfıra bölme durumunda hata mesajı verir.

Çalıştırması:

Çalıştırma: mit-scheme indirip çalıştırdım. 
mit-scheme
(load "path/to/calculator") //calculatorın pathini yazmak gerekiyor

Kullanım: fonksiyonu çağırarak hesaplama yapabilir

(calculator '(* (+ 3 4) 2))

Prolog:
calculator_prolog.pl

Uygulama Açıklaması: basit aritmetik ifadeleri değerlendirir.

İfadeler: Aritmetik ifadeleri doğrudan Prolog içinde hesaplar.
Hata Yönetimi: sıfıra bölme durumunu kontrol eder ve hata mesajı verir.

Çalıştırılması:
Yorumlayıcı: swipl indirip çalıştırdım.

swipl
?- [calculator_prolog].
True ise devam edilir

Fonksiyonu çağırarak hesap yapılabilir.
?- calculator(10/2, Result).
5

?- halt. Yapılarak çıkılır.

Genel Notlar

Değişken Ataması: Rust ve Perl uygulamalarında, kullanıcı ifadesinde değişken ataması (x=5) desteklenmektedir.
Hata Mesajları: Her uygulama, girilen ifadede hata varsa veya sıfıra bölme gibi durumlarda uygun mesajlar vermektedir.



