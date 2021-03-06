# Иванов Кирилл 4410МО, исправление ошибках в написании на основе текстов авторов

## Запуск
* Создать папку resources и добавить в нее файлы с именами авторов, содержащие их текста
* Если есть необходимость, добавить папку cache(для ускоренных расчетов). В нее положить готовый json (формат смотреть в тесте в файле prepare_dict.rs)
* После этого вызвать функцию speller::arrange_spelling, передав ей имя существующего в resources автора и 

## Описание алгоритма
* Считывается датасет текстов автора из /resources/< author_name >.txt
* Генерируются два словаря, частотный и словарь с отношениями.
    Частотный - как часто слово встречается в тексте.
    С отношениями - для наиболее частых слов формируются списки слов, с которыми они соседствуют
* Ищется первое правильное слово и преобразуются все слова справа налево до начала, после этого все после этого правильного слова
    Это сделано для того, чтобы некорректное слово всегда могло подхватить контекст от правильного
* Для каждого некорректного слова ищется список наиболее подходящих по Левенштейну
* Если в списке больше двух слов, ищутся наиболее подходящие слова исходя из словаря отношений и соседей слова
* Если после авторской проверки исправление не найдено, берется наиболее часто встречающееся в тексте слово из оставшихся кандидатов

## Описание файлов программы
* ```/resources/``` - сюда кладутся файлы с исходными текстами авторов
* ```/caches/``` - сюда кладутся файлы с кэшами расчетов словарей

* ```main.rs``` - входная точка
* ```speller.rs``` - основной алгоритм
* ```levenstein.rs``` - поиск подходящих исправлений по Левенштейну
* ```author_check.rs``` - поиск подходящих исправлений по контексту(словарь отношений + соседи слова в переданном тексте)
* ```prepare_dict.rs``` - подготовка словарей, считывание из кэша
* ```freq_check.rs``` - поиск подходящих исправлений по частоте вхождения в текст
* ```types.rs``` и ```utils.rs``` - вспомогательные утилиты и типы



