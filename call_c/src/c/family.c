#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#include "family.h"


Person* create_person(const char* name, int age) {
    if (name == NULL) return NULL;

    Person* p = (Person*)malloc(sizeof(Person)); 
    if (p == NULL) return NULL;

    p->name = (char*)malloc(sizeof(name));
    if (p->name == NULL) {
        free(p);
        return NULL;
    }

    strncpy(p->name, name, strlen(name));

    p->age = age;
    return p;
}

void free_person(Person* p) {
    if (p == NULL) return;
    if (p->name != NULL) {
        free(p->name);
    }

    free(p);
}

void display_person(Person* p) {
    if (p == NULL || p->name == NULL) return;
    printf("Person: name=%s, age=%d\n", p->name, p->age);
}

Home* create_home(const char* name, int cnt) {
    if (name == NULL) return NULL;

    Home* h = (Home*)malloc(sizeof(Home));
    if (h == NULL) return NULL;

    h->member_cnt = cnt;
    h->name = (char*)malloc(strlen(name)+1);
    if (h->name == NULL) {
        free(h);
        return NULL;
    }

    strncpy(h->name, name, strlen(name));

    h->members = (Person**)malloc(sizeof(Person*) * cnt);
    if (h->members == NULL) {
        free(h->name);
        free(h);
        return NULL;
    }

    memset(h->members, 0, sizeof(Person*) * cnt);

    return h;
}

void free_home(Home* h) {
    if (h == NULL) return;

    if (h->name != NULL) {
        free(h->name);
    }

    if (h->members != NULL) {
        for (int i=0; i<h->member_cnt; i++) {
            if (h->members[i] != NULL) {
                free(h->members[i]);
            }
        }

        free(h->members);
    }

    free(h);
}

bool add_person(Home* h, Person* p) {
    if (p == NULL || h == NULL) return false;

    for(int i=0; i < h->member_cnt; i++) {
        if (h->members[i] == NULL) {
            h->members[i] = p;
            return true;
        }
    }

    return false;
}

void display_home(Home* h) {
    if (h == NULL || h->name == NULL) return;

    printf("This is the %s:\n", h->name);

    int i = 0;
    while(i < h->member_cnt && h->members[i] != NULL) {
        display_person(h->members[i]);
        i++;
    }
}
