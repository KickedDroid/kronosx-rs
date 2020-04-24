file(REMOVE_RECURSE
  "libdecrepit.a"
  "libdecrepit.pdb"
)

# Per-language clean rules from dependency scanning.
foreach(lang C)
  include(CMakeFiles/decrepit.dir/cmake_clean_${lang}.cmake OPTIONAL)
endforeach()
