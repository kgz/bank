import React, { useContext, useEffect } from 'react';
import { Alert, Button, Checkbox, Form, Input } from 'antd';
import { userContext } from '../../../..';
import { User } from '../../../../types/User';


const onFinishFailed = (errorInfo: any) => {
    console.log('Failed:', errorInfo);
};




const BasicInfo = () => {
    const {user, setUser} = useContext(userContext);
    const [errors, setErrors] = React.useState({});
    const onFinish = (values: any) => {
        const controller = new AbortController();
        const signal = controller.signal;
        const headers = {
            'Content-Type': 'application/json',
        }
        setErrors({});
        fetch('/api/me/update', {
            method: 'POST',
            headers: headers,
            body: JSON.stringify(values),
            signal: signal
        })
            .then(res => {
                if (res.status === 200) {
                    return res.json();
                }

                return res.json().then((err: any) => {
                    setErrors(err);
                    throw new Error(err);
                });
            })
            .then(res => {
                console.log(res);
                //update user.data.username to res.UserBasicForm.username
                setUser((prev: User) => {
                    return {
                        ...prev,
                        data: {
                            ...prev.data,
                            username: res.UserBasicForm.username
                        }
                    }
                })
                
            })
            .catch(err => {
                // console.log(err.response.data);
                // // get etext
                // setErrors(err.response.data);
                
            })
    }


    useEffect(() => {
        console.log(Object.values(errors));
    }, [errors])

    return (
        <Form
            name="basic"
            size="large"
            labelCol={{ span: 8 }}
            wrapperCol={{ span: 16 }}

            fields={
                [
                    {
                        name: ['name'],
                        value: Math.random().toString(36).substring(7)
                    },
                    {
                        name: ['email'],
                        value: "test@test.com"
                    },
                    {
                        name: ['validation_type'],
                        value: "userBasic"
                    }
                ]
            }
            onFinish={onFinish} 
            autoComplete="off"
            layout='vertical'
        >
            {Object.values(errors)?.map((x: any, y:any) => {
                return <Alert type="error" key={'bas_' + y} message={x} showIcon style={{marginBottom:15}}/> 
            })}
            <Form.Item
                name="username"
                rules={[{ required: true, message: 'Please input your username!' }]}
            >
                <Input addonBefore = "Display Name"/>
            </Form.Item> 

            <Form.Item
                name="email"
                rules={[
                    {
                        type: 'email',
                        message: 'The input is not valid E-mail!',
                    },
                    {
                        required: true,
                        message: 'Please input your E-mail!',
                    },
                ]}
            >
                <Input addonBefore="Email"/>
            </Form.Item>

            <Form.Item wrapperCol={{ offset: 8, span: 16 }}>
                <Button type="primary" htmlType="submit">
                    Submit
                </Button>
            </Form.Item>
            <Form.Item name='validation_type'  style={{ display: 'none' }}>
                <Input type='hidden' />
            </Form.Item>
        </Form>
    )
}

export default BasicInfo;