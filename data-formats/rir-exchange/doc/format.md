RIR Statistics Exchange Format
--------------------------------

Sources:
* [RIR Stats Format](http://www.apnic.net/db/rir-stats-format.html)
* [README](https://ftp.arin.net/pub/stats/arin/README)

Type: CSV File, with `#` as a comment, and `|` as a field separator, and `\n` as a line separator.

Version Line:
`version|registry|serial|records|startdate|enddate|UTCoffset`
* Version: format version of the file, currently `2`
* Registry: as for records and filename (see below)
* Serial: Serial number of the file (within the creating RIR series)
* Records: number of data records in the file, excluding version & summary lines
* StartDate: start date of time period, `YYYYMMDD` format
* EndDate: end date of time period, `YYYYMMDD` format
* UTCOffset: offset from utc (+/- hours) of the local RIR producing file, `+HHMM` or `-HHMM`

Summary Lines:
`registry|*|type|*|count|summary`
* Registry: as for records (see below)
* `*` means unused/retired field
* Type: as for records (see below)
* Count: Sum of the number of record lines of this type in the file
* Summary: ASCII String `summary` to distinguish the record line

Record format:
`registry|cc|type|start|value|date|status[|extensions...]`
* Registry: One of `{afrinic,apnic,arin,iana,lacnic,ripencc};`
* CC: ISO 3166 2-Letter Country Code
* Type: One of `{asn,ipv4,ipv6};`;
* Start:
  * `asn`: The 16-bit or 32-bit Autonomous System Number
  * `ipv4,ipv6`: First address of the range
* Value:
  * `asn`: The count of sequential ASNs in the range
  * `ipv4`: The count of the number of hosts in the range
  * `ipv6`: The CIDR Prefix length from the first address value of `start`
* Date: Date the allocation was made by the RIR, format `YYYYMMDD`
* Status: Type of allocation from the set
* Extensions: Free form extensions 

Known Extensions:
* `opaque-id`: This is an in-series identifier which uniquely identifies a single
  organisation, an Internet number resource holder. All records in the file 
  with the same reg-id are registered to the same resource holder. The reg‐id 
  is not guaranteed to be constant between versions of the file.