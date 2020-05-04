# kronosx-rs


## Usage

### Dag 

````
fn dag_put_test() {
     let dag_service = create_dag_service();
     let data = "Hey Bruh".as_bytes();
     put_dag(dag_service, data);
}
````
````
OUTPUT: "bafkreib2wkausqyegptb7m7vhegd3oiqdzhwnhb7xxkipqifi3623u7feq"
````

````
fn dag_get_test() {
    let dag_service = create_dag_service();
    let cid = "bafkreib2wkausqyegptb7m7vhegd3oiqdzhwnhb7xxkipqifi3623u7feq";
    get_dag(dag_service, cid.to_string());
}
````

````
OUTPUT: Ok("Hey Bruh")
````
