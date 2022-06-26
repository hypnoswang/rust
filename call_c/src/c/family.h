#include <stdbool.h>

typedef struct Person {
    char* name;
    int age;
} Person;

typedef struct Home {
    char* name;
    int member_cnt;
    Person** members; 
} Home;

Person* create_person(const char* name, int age);
void free_person(Person* p);
void display_person(Person* p);

Home* create_home(const char* name, int cnt);
void free_home(Home* h);
bool add_person(Home* h, Person* p);
void display_home(Home* h);
