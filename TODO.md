# TODO

## Authentication
- [ ] /register is currently accepting non-encrypted (regular JSON) payloads

## Authorization
- [x] POST requests to /block should be authenticated as well (2021-04-13 04:50, they now are but until we make error messages **Verbose** there's not much point in testing because I honestly cannot trace the code)

## Verbosity
- [ ] Verbose error messages (use error.rs from [logrocket](https://blog.logrocket.com/create-an-async-crud-web-service-in-rust-with-warp/) ❓)

## Tests
- [ ] Schema Tests
- [ ] Route Tests
    - [ ] Malformed JSON bodies
    - [ ] Valid JSON with missing fields
    - [ ] Valid JSON with extra fields

## Fun (if we have time)
- [ ] use [juice](https://www.getzola.org/themes/juice/) theme with [template rendering](https://blog.logrocket.com/template-rendering-in-rust/) to create a landing page. I want it to look handmade & _scammy_, I'm talking [verylegit](https://verylegit.link) shortened urls, botched [this person does not exist](https://www.thispersondoesnotexist.com/) user stories etc.
- [ ] CHAOS MODE, 3 different coins, combine them to make 1 gradecoin

## Done & Brag
- [x] Switch to RwLock (parking_lot) (done at 2021-04-07 03:43, two possible schemes to represent inner Db (ledger) in code)
- [x] We need our own representation of students and their grades, "there is no blockchain" (done at 2021-04-12 00:05)
- [x] pick a block proposal scheme (= pick hash function) [list of hash functions](https://en.bitcoinwiki.org/wiki/List_of_hash_functions) (done at 2021-04-12 05:30)
- [x] check the nonce for incoming blocks (done at 2021-04-12 05:30)
- [X] pick a user authentication scheme = [JWT](https://tools.ietf.org/html/rfc7519) Seems perfect
- [X] implement JWT
- [X] users should be able to _sign_ their transactions
