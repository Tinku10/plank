## Simple File Format (SF2)

A very simple file format for structured data

## Format Specification

```
Jack,Emily
Johnson,Clark
28,34
New York,London
Aarav,Sophia
Sharma,Miller
22,29
Delhi,Berlin
Liam
O'Connor
41
Dublin
!SCHEMA=first_name:str,last_name:str,age:str,city:str,
!OFFSETS=0,11,25,31,47,60,74,80,93,98,107,110,
!RCOUNT=6
!CCOUNT=4
!FOOTER=117
```

The example above is organized into a few sections:

1. Row Groups:
    It is a collection of a fixed chunk of rows with the columns from each row in one line.

2. Column:
    A column contains all the enties in that column in a line for the row group.

3. Footer:
    A footer contains the complete metadata. It contains the table headers along with offsets to each row group along with the offsets to each row in the rowgroup.
