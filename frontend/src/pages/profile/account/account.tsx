import { Collapse, Descriptions, Form, Input } from 'antd';
import { useContext, useEffect, useMemo, useState } from 'react';
import styles from './account.module.scss';
import Cookies from 'universal-cookie';
import Image from '../../../fetch/image';
import fetch_api from '../../../fetch/fetch';
import Modal from 'antd/es/modal/Modal';
import { userContext } from '../../..';

const Account = () => {
    const [update, setUpdate] = useState(0);
    const [isModalOpen, setIsModalOpen] = useState(false);
    const [err, setErr] = useState('');
    const { user, setUser } = useContext(userContext);
    const { Panel } = Collapse;

    const [dummyData, setDummyData] = useState({
        name: "John Doe",
        email: "t@t.com",
        img: "http://127.0.0.1:3030/me",
        path: '512/727/727399.png'
    });

    const image_hash = useMemo(() => {
    }, [dummyData.img]);


    const cookies = new Cookies();

    const handleFile = (e: any) => {
        const file = e.target.files[0];
        const reader = new FileReader();
        const Controller = new AbortController();
        const signal = Controller.signal;


        reader.onload = () => {
            if (reader.readyState === 2) {
                let headers = {
                    'Content-Type': 'application/json',
                    'Authorization': 'Bearer ' + cookies.get('token')
                }
                if (file.size > 1000000) {
                    setErr('File size is too big');
                    setIsModalOpen(true);
                    return;
                }
                fetch('/me', {
                    method: 'POST',
                    headers: headers,
                    body: JSON.stringify({
                        img: reader.result
                    }),
                    signal: signal
                })
                    .then(res => {
                        if (res.status === 200) {
                            setUser({ ...user, profileCache: new Date().getTime().toString() });
                            // setUpdate(update+1);
                            return res.text();
                        }
                        throw new Error(res.statusText);

                    })
                    .catch(err => {
                        console.log(err);
                        setErr(err.message);
                        setIsModalOpen(true);
                    })


                setDummyData({
                    ...dummyData,
                    img: reader.result as string,
                    path: file.name
                })
            }
        }
        reader.readAsDataURL(file);
    }

    return (
        <div className={styles.wrapper}>
            <div className={styles.account_logo}>
                <div className={styles.account_logo__img}>
                    <Image key={update} src={"/me"} alt="profile" />
                </div>
                <div className={styles.account_logo__name}>
                    <span className={styles.desc}>
                        Upload a photo that identifies you
                    </span>
                    <br />
                    <p>{dummyData.path}</p>
                </div>
                {/* uplaod button */}
                <div className={styles.account_logo__upload}>
                    <button onClick={() => {
                        document.getElementById('file')?.click();
                    }}>Upload</button>
                </div>
                <input type="file" name="file" id="file" className={styles.inputfile}
                    onChange={handleFile}
                    accept="image/*"
                />
            </div>

            <div className={styles.account_form}>

                <Collapse style={{background:'white'}}>
                    <Panel header="Basic Infomation" key="1" >
                
                        <Input addonBefore={"Display Name"} placeholder={dummyData.name}  />


                        {/* <Form >
                            <Form.Item label="Name" name="name">
                                <input type="text" placeholder={dummyData.name} />
                            </Form.Item>
                            <Form.Item label="Email" name="email">
                                <input type="text" placeholder={dummyData.email} />
                            </Form.Item>
                            <Form.Item label="Password" name="password">
                                <input type="password" placeholder="********" />
                            </Form.Item>
                            <Form.Item label="Confirm Password" name="confirm_password">
                                <input type="password" placeholder="********" />
                            </Form.Item>
                            <Form.Item>
                                <button>Save</button>
                            </Form.Item>
                        </Form> */}

                    </Panel>
                    <Panel header="Security" key="2">
                        <p>Content of Panel 2</p>
                    </Panel>
                    <Panel header="Privacy" key="3">
                        <p>Content of Panel 3</p>
                    </Panel>

                </Collapse>
            </div>
            <Modal title="Basic Modal" open={isModalOpen} onOk={() => setIsModalOpen(false)} cancelText={false}>
                <p>{err}</p>
            </Modal>

        </div>


    );
};

export default Account;