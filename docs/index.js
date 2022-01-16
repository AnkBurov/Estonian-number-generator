import init, {to_letters_js} from "./pkg/estonian_numbers_generator.js";

var inited = init("./pkg/estonian_numbers_generator_bg.wasm");

var index = {
    async to_letters(bigInt, question_number) {
        await inited;
        var number = bigInt.toString();
        return to_letters_js(number, question_number);
    },
};

export default index
