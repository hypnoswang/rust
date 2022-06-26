#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Family Family;

typedef struct Person Person;

struct Person *create_person(const char *name, unsigned int age);

void free_person(struct Person *p);

struct Family *create_family(const char *name, unsigned int count);

void free_family(struct Family *f);

void add_person(struct Family *f, const struct Person *p);

void display_family(const struct Family *f);
