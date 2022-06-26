#include <stdio.h>

#include "../called_by_rust/family.h"

int main() {
    char hn[] = "Simpsons";

    Home *h = create_home(hn, 3);

    char homer[] = "Homer";
    Person* hp = create_person(homer, 45);
    add_person(h, hp);

    char lisa[] = "Lisa";
    Person* hl = create_person(lisa, 9);
    add_person(h, hl);

    char bart[] = "Bart";
    Person* hb = create_person(bart, 7);
    add_person(h, hb);

    char maggie[] = "Maggie";
    Person* hm = create_person(maggie, 4);
    add_person(h, hm);

    display_home(h);

    free_home(h);
}
