class MyClass
{
    public:
        MyClass(int initial_value) : value(initial_value) {}
        int get_value() const {return value;}
        
    private:
        int value;    
};