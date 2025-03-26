with Ada.Text_IO;
with Ada.Integer_Text_IO;
use Ada.Text_IO, Ada.Integer_Text_IO;

procedure calculator is
   a: Integer;
   b : Integer;
   sonuc  : Integer;
   operator : Character;

begin
   Put_Line("bir ifade girin: ");

   Get(Item => a);

   Get(Item => operator);

   Get(Item => b);

   case operator is
      when '+' =>
         sonuc := a + b;
      when '-' =>
         sonuc := a - b;
      when '*' =>
         sonuc := a * b;
      when '/' =>
         if b /= 0 then
            sonuc := a / b;
         else
            Put_Line("hata 0a bölünme");
            sonuc := 0;
         end if;
      when others =>
         Put_Line("geçersiz");
         sonuc := 0;
   end case;

   Put("sonuç:");
   Put_Line(Integer'Image(sonuc));

end calculator;
