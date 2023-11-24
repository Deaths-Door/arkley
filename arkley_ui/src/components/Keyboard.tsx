import { Dispatch, SetStateAction } from "react";
import { View , StyleSheet, TouchableOpacity } from "react-native";
import { Card , Button , Text} from "react-native-paper";

export default function Keyboard({ input, setInput} : { input : string , setInput : Dispatch<SetStateAction<string>>}){
    type KeyWithClosure = {
        label: string;
        closure: ((label : string) => void) | null ;
    };

    // TODO : Create specialized closures for each one
    const keys: KeyWithClosure[][] = [
        [
          { label: 'AC', closure: () => setInput("0") },
          { label: '()', closure: () => console.log('Default () button clicked') },
          { label: '%', closure: () => console.log('Default % button clicked') },
          { label: '÷', closure: null },
        ],
        ['7', '8', '9', '×'].map(label => ({ label, closure: null })),
        ['4', '5', '6', '-'].map(label => ({ label, closure: null })),
        ['1', '2', '3', '+'].map(label => ({ label, closure: null })),
        [
          { label: '±', closure: () => console.log('Default % button clicked') },
          { label: '0', closure: null },
          { label: '.', closure: null },
          { label: '=', closure: null },
        ]
    ];

    const defaultClosure = (label: string) => setInput(input == "0" ? label : input + label);
    
    return (
        <Card>{
            keys.map((inner) => 
                <View style={styles.row}>{
                    inner.map((item) => (
                        <Button style={styles.item} mode="elevated" onPress={() => (item.closure ?? defaultClosure)(item.label)}>
                            <Text variant="titleLarge" allowFontScaling={true}>{item.label}</Text>
                        </Button>
                    ))
                }</View> 
            )
        }</Card>
    )
}

const styles = StyleSheet.create({
    row  : { 
        flexDirection : 'row' , 
        justifyContent : "space-evenly",
        margin : 16
    },
    item : { borderRadius : 30 }
});