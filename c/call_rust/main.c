#include <stdio.h>

#include "family_ffi.h"

int main() {
    char* persons[] = {
        "Homer", "Marge", "Bart", "Lisa", "Maggie"
    };

    Family* f = create_family("Simpsons", 6);

    for (int i=0; i<(sizeof(persons)/sizeof(char*)); i++) {
       Person* p = create_person(persons[i], 14+i); 
       add_person(f, p);
       free_person(p);
    }

    display_family(f);

    free_family(f);
}
