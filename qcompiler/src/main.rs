extern crate pest;
#[macro_use]
extern crate pest_derive;

mod parsing;
mod codegen;
use crate::parsing::basic_parsing;


fn main() {
    
}

#[cfg(test)]
mod tests {
    /* IMPORTS
     * Used in testing
     */
    use crate::codegen::{
        QyriData,
        QyriEnumeration,
        QyriStructure,
        QyriType,
        QyriAtom,
        QyriPrimitive,
    };

    /* TEST
     * Creates an enumeration of integers and lists those variants
     */
    #[test]
    fn create_primitive_enumeration_and_list_variants() {
        let enumer = QyriEnumeration::new(vec![
            QyriType::primitive_(
                Box::from(
                    QyriPrimitive::int_(
                        QyriAtom::new(&[0x00, 0x00, 0x00, 0x01])
                    )
                )
            ),

            QyriType::primitive_(
                Box::from(
                    QyriPrimitive::int_(
                        QyriAtom::new(&[0x00, 0x00, 0x00, 0x02])
                    )
                )
            ),
        ]);
        for item in enumer.get_data() {
            match &**item {
                QyriType::primitive_(i) => {
                    match &**i {
                        QyriPrimitive::int_(a) => println!("{:?}", a),
                        _ => unreachable!(),
                    };
                },
                _ => unreachable!(),
            };
        }
    }
}