import wordsegment

wordsegment.load()
res = wordsegment.segment('TRUENERVOUSVERYVERYDREADFULLYNERVOUSIHADBEENANDAMBUTWHYWILLYOUSAYTHATIAMMADTHEDISEASEHADSHARPENEDMYSENSESNOTDESTROYEDNOTDULLEDTHEMABOVEALLWASTHESENSEOFHEARINGACUTEIHEARDALLTHINGSINTHEHEAVENANDINTHEEAR')

print(' '.join(res))