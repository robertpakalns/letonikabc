# LetonikaBC
Letonika Better Client 

Tīmekļa vietne, kurā lietotājs lejupielādē teksta failu no [letonika.lv](https://letonika.lv) vietnes un izmanto kā alternatīvu lasītāvu.

## Sistēmas vienkāršots modelis  

###  Modeļa objekti  

* Teksta fails no `letonika.lv`  
Sākotnējais saturs, ko lietotājs importē no savas ierīces un izmanto lasīšanai
* Apstrādāts dokuments  
Sistēmas izveidots, lietošanai pielāgots dokuments, ar kuru lietotājs strādā vietnē
* Meklēšanas filtrs  
Modulis, kas ļauj lietotājam atrast tekstu dokumentā
* Iebūvētā datubāze  
Pārlūkprogrammas lokālā krātuve, kur tiek saglabāti dokumenti un informācija par tiem
* Dokumentu apstrādes modulis  
Iekšējais sistēmas mehānisms, kas pārveido sākotnējo tekstu lietotājam draudzīgā formā

### Modeļa objektu saites
...

## Datu vārdnīca

### Oriģināls teksta fails
| Lauks  | Tips   | Apraksts                 |
| ------ | ------ | ------------------------ |
| ID     | number | Dokumenta identifikators |
| Izmērs | number | Rakstzīmju skaits        |
| Saturs | string | Pilns teksta saturs      |

### Apstrādāts dokuments
| Lauks     | Tips   | Apraksts           |
| --------- | ------ | ------------------ |
| ID        | number | Dokumenta ID       |
| Izmērs    | number | Teksta garums      |
| Saturs    | string | Saglabātais teksts |
| Nosaukums | string | Var būt tukšs      |
| Autors    | string | Var būt tukšs      |

### Metadati
| Lauks               | Tips   | Apraksts                    |
| ------------------- | ------ | --------------------------- |
| ID                  | number | Primārā atslēga             |
| Sākotnējais izmērs  | number | Oriģinālais teksta izmērs   |
| Apstrādātais izmērs | number | Teksta izmērs pēc apstrādes |
| Nosaukums           | string | Var būt tukšs               |
| Autors              | string | Var būt tukšs               |
| Izveides laiks      | Date   | ISO 8601 standarts          |
| Rediģēšanas laiks   | Date   | ISO 8601 standarts          |

## Funkcionālās prasības
1. Sistēma ļauj lietotājam augšupielādēt teksta failu no savas ierīces.
2. Sistēma pārbauda, vai fails ir derīgs turpmākai apstrādei.
3. Sistēma sagatavo failu lietošanai un parāda to lietotāja saskarnē.
4. Sistēma automātiski iegūst dokumenta pamatinformāciju (ID, nosaukumu, autoru, izmēru utt.).
5. Sistēma saglabā dokumentu iebūvētajā datubāzē turpmākai lietošanai.
6. Sistēma parāda lietotājam saglabāto dokumentu sarakstu.
7. Sistēma ļauj lietotājam atvērt izvēlēto dokumentu lasīšanas režīmā.
8. Sistēma nodrošina teksta meklēšanu dokumenta saturā.
9. Sistēma ļauj lietotājam ierobežot meklēšanu uz izvēlētām teksta daļām.
10. Sistēma ļauj lietotājam labot dokumenta nosaukumu un autoru.
11. Sistēma ļauj importēt un eksportēt dokumentus `.md` formātā.
