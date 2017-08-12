/* generated by rust_qt_binding_generator */
#ifndef TEST_OBJECT_TYPES_RUST_H
#define TEST_OBJECT_TYPES_RUST_H

#include <QObject>
#include <QVariant>
#include <QAbstractItemModel>

class ObjectInterface;
class Object : public QObject
{
    Q_OBJECT
    ObjectInterface * const d;
    Q_PROPERTY(bool boolean READ boolean WRITE setBoolean NOTIFY booleanChanged FINAL)
    Q_PROPERTY(int integer READ integer WRITE setInteger NOTIFY integerChanged FINAL)
    Q_PROPERTY(uint uinteger READ uinteger WRITE setUinteger NOTIFY uintegerChanged FINAL)
    Q_PROPERTY(QString string READ string WRITE setString NOTIFY stringChanged FINAL)
    Q_PROPERTY(QByteArray bytearray READ bytearray WRITE setBytearray NOTIFY bytearrayChanged FINAL)
public:
    explicit Object(QObject *parent = nullptr);
    ~Object();
    bool boolean() const;
    void setBoolean(bool v);
    int integer() const;
    void setInteger(int v);
    uint uinteger() const;
    void setUinteger(uint v);
    QString string() const;
    void setString(const QString& v);
    QByteArray bytearray() const;
    void setBytearray(const QByteArray& v);
signals:
    void booleanChanged();
    void integerChanged();
    void uintegerChanged();
    void stringChanged();
    void bytearrayChanged();
private:
    bool m_boolean;
    int m_integer;
    uint m_uinteger;
    QString m_string;
    QByteArray m_bytearray;
};
#endif // TEST_OBJECT_TYPES_RUST_H
