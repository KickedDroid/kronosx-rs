# kronosx-rs


## Usage

### Dag put
```rust
fn dag_put_test() {
     let data = "Hey Bruh".as_bytes();
     put_dag(dag_service, data);
}
````
````
OUTPUT: "bafkreib2wkausqyegptb7m7vhegd3oiqdzhwnhb7xxkipqifi3623u7feq"
````
### Dag get
```rust
fn dag_get_test() {
    let cid = "bafkreib2wkausqyegptb7m7vhegd3oiqdzhwnhb7xxkipqifi3623u7feq";
    get_dag(dag_service, cid.to_string());
}
````

````
OUTPUT: Ok("Hey Bruh")
````
### Dag get many
```rust
fn dag_get_many_test() {
    let cid = "bafkreib2wkausqyegptb7m7vhegd3oiqdzhwnhb7xxkipqifi3623u7feq";
    let cid2 = "bafkreiadrrjhot4osxtgb6voecvgfwabmjesgfdblh6iqgqt3l6l6soz4y";

    let mut cids = Vec::default();
    cids.push(cid.to_string());
    cids.push(cid2.to_string());
    get_dag_many(&cids);
}
````
````
OUTPUT: 
Ok("Hey Bruh")
Ok("Wassaup dude")
````
