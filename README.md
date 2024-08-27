# Common Programming Concepts
## Difference between `mut` and shadowing

|     | `mut`                                                                                                        | shadowing                                                                                                                                                                                              |
| --- | ------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 1.  | Allows to make a variable mutable, meaning you can change its value after it has been initialized, not type. | Allows to re-declare a variable with the same name in the same scope, effectively *shadowing* the previous variable, new variable can be of different type, mutable/immutable, regardless of original. |
| 2.  | Applies only to the specific variable within its current scope.                                              | Shadowed variable is hidden within the scope the new variable is declared. The previous variable is inaccessible until the shadowing variable goes out of scope.                                       |
| 3.  | You use `mut` when you need to modify the values of a variable multiple times throughout its lifetime.       | You use shadowing when you want to reuse a variable name but with a different type or value, or when you want to apply some transformation without mutating the original variable.                     |


