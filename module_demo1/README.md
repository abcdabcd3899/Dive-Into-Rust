## Module System

### Example

```shell
crate
   |--mod front_of_house -> front_of_house.rs
      |-----mod hosting (front_of_house/hosting.rs)
      |     |----add_to_waitlist
      |     |----seat_at_table
      |-----mod serving (front_of_house/serving.rs)
      |     |----take_order
      |     |----serve_order
      |     |----take_payment
      |     |----mod back_of_house (serving/back_of_house.rs)
```